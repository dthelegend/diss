use std::fmt::{Debug, Formatter};
use std::iter::zip;

use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::{CooMatrix, CsrMatrix, SparseFormatError};
use thiserror::Error;

#[cfg(test)]
mod test;

pub mod solver;

pub mod helpers;

pub type QuboType = i32;

pub struct QuboProblem(CsrMatrix<QuboType>, usize);

#[derive(Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct QuboSolution(pub DVector<QuboType>);

impl QuboSolution {
    fn flip(&self, i: usize) -> Self {
        let mut temp_solution = self.clone();

        temp_solution.0[i] = 1 - temp_solution.0[i];

        temp_solution
    }
}

#[derive(Error, Debug)]
pub enum QuboError {
    #[error("The provided Q Matrix has an invalid triplet: {0}")]
    InvalidTriplets(#[from] SparseFormatError),
    #[error("The provided Q Matrix has a non-square size")]
    IncorrectSize
}

impl QuboProblem {
    pub fn try_from_q_matrix(q_matrix: CsrMatrix<QuboType>) -> Result<Self, QuboError> {
        let n_rows = q_matrix.nrows();
        if n_rows != q_matrix.ncols() {
            Err(QuboError::IncorrectSize)
        } else {
            let modified_q_matrix = q_matrix.transpose() - q_matrix.diagonal_as_csr() + q_matrix;
            Ok(QuboProblem(modified_q_matrix, n_rows))
        }
    }

    pub fn try_from_coo_matrix(coo_matrix: &CooMatrix<QuboType>) -> Result<Self, QuboError> {
        QuboProblem::try_from_q_matrix(CsrMatrix::from(coo_matrix))
    }

    pub fn try_from_triplets(
        problem_size: usize,
        triplets: Vec<(usize, usize, QuboType)>,
    ) -> Result<Self, QuboError> {
        let (row_indices, col_indices, values) = {
            let trip_len = triplets.len();
            triplets.into_iter().fold(
                (
                    Vec::with_capacity(trip_len),
                    Vec::with_capacity(trip_len),
                    Vec::with_capacity(trip_len),
                ),
                |(mut i_list, mut j_list, mut v_list), (i, j, v)| {
                    i_list.push(i);
                    j_list.push(j);
                    v_list.push(v);

                    (i_list, j_list, v_list)
                },
            )
        };

        let m = CooMatrix::try_from_triplets(
            problem_size,
            problem_size,
            row_indices,
            col_indices,
            values,
        )?;

        QuboProblem::try_from_coo_matrix(&m)
    }

    pub fn try_from_ising_triplets(
        problem_size: usize,
        j_triplets: Vec<(usize, usize, QuboType)>,
        j_biases: Vec<(usize, QuboType)>,
    ) -> Result<(Self, QuboType), QuboError> {
        let mut q_matrix = CooMatrix::new(problem_size, problem_size);

        let mut offset = 0;
        for (i, b) in j_biases {
            q_matrix.push(i, i, 2 * b);
            offset -= b;
        }

        for (i, j, b) in j_triplets {
            if b == 0 {
                continue;
            }
            q_matrix.push(i, j, 4 * b);
            q_matrix.push(i, i, -2 * b);
            q_matrix.push(j, j, -2 * b);

            offset += b;
        }

        QuboProblem::try_from_coo_matrix(&q_matrix)
            .map(|x| (x, offset))
    }

    pub fn get_size(&self) -> usize {
        self.1
    }

    pub fn evaluate(&self, QuboSolution(solution_vector): &QuboSolution) -> QuboType {
        let QuboProblem(q_matrix, _) = self;

        // Matrix math is associative, and only csr * dense is implemented
        let xqx = solution_vector.transpose() * (q_matrix * solution_vector);
        *xqx.get((0, 0))
            .expect("If dimensions match the final matrix is a 1x1 matrix")
    }

    /// Returns the difference between the `E(f(k, f(j, X)))` and `E(f(k, X))`. i.e. the delta obtained
    /// by flipping j from f(X, k) (knowing D(k)).
    /// `E(f(k, X)) + result = E(f(k, f(j, X)))`
    /// This would be O(1) for a statically allocated array, but the CSR used to optimise
    /// memory means that this operation is actually O(log n).
    ///
    /// # Arguments
    ///
    /// * `solution`: The original solution X
    /// * `delta_k`: E(flip(k, X)) - E(X)
    /// * `k`: The bit flipped that created the delta k
    /// * `j`: The bit to flip to produce the solution
    ///
    /// returns: isize
    ///
    /// # Examples
    ///
    /// ```
    /// let sut = ...;
    ///
    /// let sut_solution = ...;
    /// let eval = sut.evaluate(&sut_solution);
    ///
    ///
    /// // Delta(k, X)
    /// let sut_solution_k = sut_solution.flip(k);
    /// let eval_k = sut.evaluate(&sut_solution_k);
    /// let delta_k = eval_k - eval;
    ///
    /// // D(k, f(j, X)) = E(f(k, f(j, X)) - E(f(j, X))
    /// let delta_eval_k_and_eval_j =
    /// //                              X              D(k, X) j  k
    /// sut.flip_j_and_delta_evaluate_k(&sut_solution, delta_k,    j, k);
    ///
    /// assert_eq!(sut.evaluate(&sut_solution.flip(k).flip(j)) - sut.evaluate(&sut_solution.flip(j)), delta_eval_k_and_eval_j);
    /// ```
    pub fn flip_j_and_delta_evaluate_k(
        &self,
        solution: &QuboSolution,
        delta_k: QuboType,
        j: usize,
        k: usize,
    ) -> QuboType {
        if j == k {
            return -delta_k;
        }

        let w_jk = self
            .0
            .get_entry(j, k)
            .expect("J and K should not be out of bounds!")
            .into_value();

        let s_j = helpers::sigma(solution, j);
        let s_k = helpers::sigma(solution, k);

        delta_k + 2 * w_jk * s_j * s_k
    }

    /// Calculate the delta between a solution and the next solution by flipping bit k
    /// This operation happens in O(n)
    ///
    /// # Arguments
    ///
    /// * `solution`: The solution
    /// * `k`: The kth bit to flip
    ///
    /// returns: isize
    ///
    /// # Examples
    ///
    /// ```
    /// let sut = ...;
    /// let sut_solution = ...;
    ///
    /// let eval = sut.evaluate(&sut_solution);
    ///
    /// let sut_solution_k = ...;
    /// let eval_k = sut.evaluate(&sut_solution_k);
    ///
    /// let delta_k = sut.delta_evaluate_k(&sut_solution, k);
    ///
    /// assert_eq!(eval_k - eval, delta_k);
    /// ```
    pub fn delta_evaluate_k(
        &self,
        solution @ QuboSolution(solution_vector): &QuboSolution,
        k: usize,
    ) -> QuboType {
        let row = self.0.get_row(k).expect("K should not be out of bounds!");

        let row_sum: QuboType = zip(
            row.col_indices().iter().cloned(),
            row.values().iter().cloned(),
        )
        .map(|(i, x)| solution_vector[i] * x)
        .sum();

        let sigma_k = helpers::sigma(solution, k);

        let w_kk = row.get_entry(k).unwrap().into_value();

        -2 * row_sum * sigma_k + w_kk
    }
}

/// Note: This operation is expensive, only print if ABSOLUTELY necessary
impl Debug for QuboProblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "QuboProblem of size {}{}",
            self.1,
            self.0.clone() * DMatrix::identity(self.1, self.1)
        )
    }
}

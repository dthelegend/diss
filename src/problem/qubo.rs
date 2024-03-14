use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::CsrMatrix;
use std::fmt::{Debug, Formatter};
use std::iter::zip;
use thiserror::Error;

#[cfg(test)]
mod test;

pub mod solver;

pub type QuboType = isize;

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
    #[error("The provided Q Matrix has a non-square size")]
    IncorrectSize,
    #[error("The provided Q Matrix is not upper triangular")]
    NotTriangular,
}

impl QuboProblem {
    pub fn try_from_q_matrix(q_matrix: CsrMatrix<QuboType>) -> Result<Self, QuboError> {
        let n_rows = q_matrix.nrows();
        if n_rows != q_matrix.ncols() {
            Err(QuboError::IncorrectSize)
        } else if q_matrix.upper_triangle() != q_matrix {
            Err(QuboError::NotTriangular)
        } else {
            let modified_q_matrix = q_matrix.transpose() - q_matrix.diagonal_as_csr() + q_matrix;
            Ok(QuboProblem(modified_q_matrix, n_rows))
        }
    }

    pub fn get_size(&self) -> usize {
        self.1
    }

    pub fn evaluate(&self, QuboSolution(solution_vector): &QuboSolution) -> QuboType {
        let QuboProblem(q_matrix, _) = self;

        // Matrix math is associative, and only csr * dense is implemented
        let xqx =
            solution_vector.clone().cast::<isize>().transpose() * (q_matrix * solution_vector);
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
        QuboSolution(solution_vector): &QuboSolution,
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

        let s_j = 2 * solution_vector[j] - 1;
        let s_k = 2 * solution_vector[k] - 1;

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
        QuboSolution(solution_vector): &QuboSolution,
        k: usize,
    ) -> QuboType {
        let row = self.0.get_row(k).expect("K should not be out of bounds!");

        let row_sum: QuboType = zip(
            row.col_indices().iter().cloned(),
            row.values().iter().cloned(),
        )
        .map(|(i, x)| solution_vector[i] * x)
        .sum();

        let sigma_k = 2 * solution_vector[k] - 1;

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

use nalgebra::DVector;
use nalgebra_sparse::CsrMatrix;
use thiserror::Error;

#[cfg(test)]
mod test;

pub mod solver;

type QuboType = isize;

pub struct QuboProblem(CsrMatrix<QuboType>, usize);

#[derive(Clone)]
pub struct QuboSolution(pub DVector<QuboType>);

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
            Ok(QuboProblem(q_matrix, n_rows))
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
}

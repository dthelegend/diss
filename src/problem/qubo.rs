use nalgebra::DVector;
use nalgebra_sparse::CsrMatrix;

#[cfg(test)]
mod test;

pub mod solver;

pub struct QuboProblem(CsrMatrix<i32>);

pub struct QuboSolution(DVector<i32>);

impl QuboProblem {
    pub fn evaluate(&self, QuboSolution(solution_vector): &QuboSolution) -> i32 {
        let QuboProblem(q_matrix) = self;

        // Matrix math is associative, and only csr * dense is implemented
        let xqx = solution_vector.transpose() * (q_matrix * solution_vector);
        *xqx.get((0,0))
            .expect("If dimensions match the final matrix is a 1x1 matrix")
    }
}
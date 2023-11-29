use std::{ops::{Index, IndexMut}, iter::zip, fmt::Debug};
use crate::matrix;

use super::Problem;

#[derive(Debug)]
pub enum QuboProblemBackend {
    Parallel,
    Sequential
}

#[derive(Debug)]
pub struct QuboProblem {
    problem_matrix: matrix::SparseMatrix<i32>,
    problem_backend: QuboProblemBackend
}

pub struct QuboSolution(pub Vec<bool>);

impl Problem<QuboSolution, i32> for QuboProblem {
    fn validate_solution(&self, solution: &QuboSolution) -> i32 {
        let QuboSolution(solution_vector) = solution;
        assert!(solution_vector.len() == self.get_size());

        let mut x_q = vec![0; solution_vector.len()];

        // Vector multiply Sparse matrix
        for &matrix::SparseMatrixElement { row, column, value } in self.problem_matrix.values() {
            x_q[column] += value * (if solution_vector[row] { 1 } else { 0 });
        }

        // multiply two matrices
        zip(x_q, solution_vector).map(|(a, b)| a * (if *b { 1 } else { 0 })).sum()
    }

    fn solve(&self) -> QuboSolution {
        match self.problem_backend {
            QuboProblemBackend::Parallel => todo!(),
            QuboProblemBackend::Sequential => todo!(),
        }
    }
}

impl QuboProblem {
    pub fn new(size: usize) -> Self {
        Self::new_with_backend(size, QuboProblemBackend::Sequential)
    }

    pub fn new_with_backend(size: usize, backend: QuboProblemBackend) -> Self {
        QuboProblem { problem_matrix: matrix::SparseMatrix::new((size, size)), problem_backend: backend }
    }

    fn adjust_index(index: (usize, usize)) -> (usize, usize) {
        let (x, y) = index;
        if x > y {
            (y, x)
        }
        else {
            (x,y)
        }
    }

    pub fn get_size(&self) -> usize {
        let QuboProblem { problem_matrix, .. } = self;
        let (x, y) = problem_matrix.shape;
        
        assert!(x == y, "Problem is not square!");
        
        x
    }
}

// QUBO Indexing
impl IndexMut<(usize, usize)> for QuboProblem {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.problem_matrix[Self::adjust_index(index)]
    }
}

impl Index<(usize, usize)> for QuboProblem {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.problem_matrix[Self::adjust_index(index)]
    }
}

// DEBUG

impl Debug for QuboSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUBOSolution ")?;
        f.debug_list().entries(self.0.iter().map(|x| if *x { 1 } else { 0 })).finish()
    }
}
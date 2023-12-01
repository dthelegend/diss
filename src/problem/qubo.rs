use std::{ops::{Index, IndexMut}, iter::zip, fmt::Debug};
use log::{trace, debug};
use crate::matrix;
use rand::{Rng, thread_rng, distributions::{weighted::WeightedIndex, Distribution}};

use super::Problem;

#[derive(Debug)]
pub enum QuboProblemBackend {
    ParallelExhaustiveSearch,
    MopsoParallel,
    MomentumAnnealing,
    SimulatedQuantumAnnealing,
    DiverseAdaptiveBulkSearch,
    SimulatedAnnealing
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
            QuboProblemBackend::ParallelExhaustiveSearch => todo!(),
            QuboProblemBackend::MopsoParallel => todo!(),
            QuboProblemBackend::MomentumAnnealing => todo!(),
            QuboProblemBackend::SimulatedQuantumAnnealing => todo!(),
            QuboProblemBackend::DiverseAdaptiveBulkSearch => todo!(),
            QuboProblemBackend::SimulatedAnnealing => {
                // Could likely be improved with a solution cache, but works well enough for demonstrative purposes
                const K_MAX: i32 = 1000;
                fn anneal_helper(problem: &QuboProblem, solution: QuboSolution, k: i32) -> QuboSolution {
                    debug!("Current solution: {:?}", solution);

                    if k <= 0 {
                        return solution;
                    }

                    let temperature: f64 = (f64::from(k)) / f64::from(K_MAX);
                    trace!("Current Temperature: {}", temperature);

                    let (mut neighbours, evals) : (Vec<_>, Vec<_>) = (0..solution.0.len())
                        .map(|i| {
                            let mut vec = solution.0.clone();
                            vec[i] = !vec[i];
                            QuboSolution(vec)
                        })
                        .map(|x| {
                            let validate_solution = problem.validate_solution(&x);
                            (x, validate_solution)
                        })
                        .unzip();

                    // Softmaxed weights because I can lol
                    let min_eval = evals.iter().min()
                        .expect("Neighbours should never be is empty unless solution is empty, and solution should never be empty!");
                    let max_eval = evals.iter().max()
                        .expect("Neighbours should never be is empty unless solution is empty, and solution should never be empty!");
                    let weights_exp = evals.iter()
                    .map(|x| {
                        let norm_x: f64 = -1.0 + 2.0 * f64::from((min_eval - x ) / (min_eval - max_eval));
                        f64::exp(-norm_x) // minimisation
                    });
                    let sum_of_exp: f64 = weights_exp.clone().sum();
                    let softmaxed_weights = weights_exp.map(|x| x / sum_of_exp);
                    let dist = WeightedIndex::new(softmaxed_weights)
                        .expect("Failed to build distribution from softmaxed values");

                    let mut rng = thread_rng();
                    let chosen_neighbour_number = dist.sample(&mut rng);

                    let chosen_neighbour = if evals[chosen_neighbour_number] < problem.validate_solution(&solution) || rng.gen_bool(temperature) {
                        neighbours.swap_remove(chosen_neighbour_number)
                    } else {
                        solution
                    };

                    anneal_helper(problem, chosen_neighbour, k-1)
                }

                let start_solution = QuboSolution((0..self.get_size()).map(|_| thread_rng().gen_bool(0.5)).collect());

                anneal_helper(self, start_solution, K_MAX)
            },
        }
    }
}

impl QuboProblem {
    pub fn new(size: usize) -> Self {
        Self::new_with_backend(size, QuboProblemBackend::SimulatedAnnealing)
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
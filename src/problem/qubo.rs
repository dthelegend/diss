use std::{ops::{Index, IndexMut}, iter::zip, fmt::{Debug, Display}};
use log::{trace, log_enabled};
use crate::matrix;
use rand::{Rng, thread_rng, distributions::{weighted::WeightedIndex, Distribution}};

use super::Problem;

#[derive(Debug, Default)]
pub enum QuboProblemBackend {
    ParallelExhaustiveSearch,
    MopsoParallel,
    MomentumAnnealing,
    SimulatedQuantumAnnealing,
    DiverseAdaptiveBulkSearch,
    #[default]
    SimulatedAnnealing
}

#[derive(Debug)]
pub struct QuboProblem {
    problem_matrix: matrix::SparseMatrix<i32>,
    problem_backend: QuboProblemBackend
}

#[derive(Clone)]
pub struct QuboSolution(pub Vec<bool>);

impl Problem<QuboSolution> for QuboProblem {
    fn solve(&self) -> QuboSolution {
        match self.problem_backend {
            QuboProblemBackend::ParallelExhaustiveSearch => todo!(),
            QuboProblemBackend::MopsoParallel => todo!(),
            QuboProblemBackend::MomentumAnnealing => todo!(),
            QuboProblemBackend::SimulatedQuantumAnnealing => todo!(),
            QuboProblemBackend::DiverseAdaptiveBulkSearch => todo!(),
            QuboProblemBackend::SimulatedAnnealing => {
                // Could likely be improved with a solution cache, but works well enough for demonstrative purposes
                const K_MAX: i32 = 10000;

                let mut solution = {
                    let rand_solution = QuboSolution((0..self.get_size()).map(|_| thread_rng().gen_bool(0.5)).collect());
                    let sol_eval = self.evaluate_solution(&rand_solution);

                    (rand_solution, sol_eval)
                };
                let mut min_solution = solution.clone();

                for k in (0..K_MAX).rev() {
                    // Linear cooling schedule.
                    // TODO: Swap for a more robust cooling schedule
                    let temperature: f64 = (f64::from(k)) / f64::from(K_MAX);

                    trace!("Current solution evaluation: {}", solution.1);
                    trace!("Current min solution evaluation: {}", min_solution.1);
                    trace!("Current Temperature: {}", temperature);

                    let (mut neighbours, evals) : (Vec<_>, Vec<_>) = (0..solution.0.0.len())
                        .map(|i| {
                            let mut vec = solution.0.0.clone();
                            vec[i] = !vec[i];
                            QuboSolution(vec)
                        })
                        .map(|x| {
                            let validate_solution = self.evaluate_solution(&x);
                            (x, validate_solution)
                        })
                        .unzip();

                    let (min_neighbour, min_eval) = evals.iter().enumerate().min_by_key(|x| x.1)
                        .expect("Neighbours should never be is empty unless solution is empty, and solution should never be empty!");
                    
                    if *min_eval < min_solution.1 {
                        min_solution = (neighbours[min_neighbour].clone(), *min_eval);
                    }
                    
                    // Stop Condition
                    // Solution will never improve if the temperature is 0 and the only options are increasing.
                    // This means we are trapped in a local minima
                    if *min_eval > solution.1 && temperature <= 0.0 {
                        break;
                    }

                    let max_eval = evals.iter().max()
                        .expect("Neighbours should never be is empty unless solution is empty, and solution should never be empty!");
                    let range = min_eval - max_eval;
                    let x : Box<dyn Fn(&i32) -> f64> = if range != 0 {
                        Box::new(|x| {
                            1.0 - f64::from((min_eval - x) / range) // 1 - x for minimisation
                        })
                    } else {
                        Box::new(|_| 1.0)
                    };
                    let weights: Vec<f64> = evals
                        .iter()
                        .map(x)
                        .collect();
                    
                    let dist = WeightedIndex::new(&weights)
                        .expect("Failed to build distribution from softmaxed values");

                    let mut rng = thread_rng();
                    let chosen_neighbour_number = dist.sample(&mut rng);

                    solution = if evals[chosen_neighbour_number] < solution.1 || rng.gen_bool(temperature) {
                        (neighbours.swap_remove(chosen_neighbour_number), evals[chosen_neighbour_number])
                    } else {
                        solution
                    };
                }

                min_solution.0
            },
        }
    }
}

impl QuboProblem {
    pub fn new(size: usize) -> Self {
        Self::new_with_backend(size, Default::default())
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

    pub fn evaluate_solution(&self, solution: &QuboSolution) -> i32 {
        let QuboSolution(solution_vector) = solution;
        assert!(solution_vector.len() == self.get_size());

        let mut x_q = vec![0; solution_vector.len()];

        // Vector multiply Sparse matrix
        for &matrix::SparseMatrixElement { row, column, value } in self.problem_matrix.values() {
            x_q[row] += value * (if solution_vector[column] { 1 } else { 0 });
        }

        // multiply two matrices
        zip(x_q, solution_vector).map(|(a, b)| a * (if *b { 1 } else { 0 })).sum()
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

// DEBUG and DISPLAY

impl Debug for QuboSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUBOSolution ")?;
        f.debug_list().entries(self.0.iter().map(|x| if *x { 1 } else { 0 })).finish()
    }
}

impl Display for QuboProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUBOProblem[Backend = {:?}] ({},{})", self.problem_backend, self.problem_matrix.shape.0, self.problem_matrix.shape.1)
    }
}
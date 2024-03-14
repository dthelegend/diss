use crate::problem::qubo::solver::QuboSolver;
use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use log::Level::Warn;
use log::{debug, log_enabled, warn};
use nalgebra::DVector;
use std::cmp::min_by_key;

pub struct ExhaustiveSearch {}

impl ExhaustiveSearch {
    pub fn new() -> Self {
        ExhaustiveSearch {}
    }
}

/// This operation is `O(n 2^n)`
fn exhaustive_search_helper(problem: &QuboProblem, solution: QuboSolution, deltas: Vec<QuboType>, curr_eval: QuboType, i: usize) -> (QuboSolution, QuboType) {
    if i == 0 {
        return (solution, curr_eval);
    }
    let solution_i = solution.flip(i - 1);
    let eval_i = curr_eval + deltas[i - 1];

    // Update deltas
    let new_deltas: Vec<_> = deltas.iter().enumerate().take(i).map(|(j, d_j)| {
        problem.flip_j_and_delta_evaluate_k(&solution, *d_j, i - 1, j)
    }).collect();

    let left_min = exhaustive_search_helper(problem, solution, deltas, curr_eval, i - 1);
    let right_min = exhaustive_search_helper(problem, solution_i, new_deltas, eval_i, i - 1);

    min_by_key(left_min, right_min, |(_, eval)| *eval)
}

impl QuboSolver for ExhaustiveSearch {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        const BIGGEST_REASONABLE_SEARCH_SIZE: usize = 32;

        let start_solution = QuboSolution(DVector::zeros(qubo_problem.get_size()));
        
        let delta_j_precalcs: Vec<QuboType> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&start_solution, i))
            .collect();

        if log_enabled!(Warn) && qubo_problem.get_size() > BIGGEST_REASONABLE_SEARCH_SIZE {
            warn!("Exhaustive Searches greater than {BIGGEST_REASONABLE_SEARCH_SIZE} can take extremely long amounts of time! (This algorithm runs in exponential time, but it is provably optimal!)")
        }

        debug!(
            "Starting search in tree of size 2^{}",
            qubo_problem.get_size()
        );

        let (min_solution, min_eval) =
            // exhaustive_search_helper(&qubo_problem, start_solution, 0, qubo_problem.get_size());
            exhaustive_search_helper(&qubo_problem, start_solution, delta_j_precalcs, 0, qubo_problem.get_size());
        
        debug!(
            "Produced a provably optimal min evaluation {} with solution: {}",
            min_eval,
            min_solution.0.transpose()
        );

        min_solution
    }
}
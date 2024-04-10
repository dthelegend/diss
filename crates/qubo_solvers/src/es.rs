use common::Solver;
use log::Level::Warn;
use log::{debug, log_enabled, warn};
use nalgebra::DVector;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use std::cmp::min_by_key;
use common::data_recorder::DataRecorder;
use serde::Serialize;

#[derive(Serialize)]
struct ExhaustiveSearchRecord {
    solution: String,
    value: QuboType
}

pub struct ExhaustiveSearch {}

impl ExhaustiveSearch {
    pub fn new() -> Self {
        ExhaustiveSearch {}
    }
}

#[inline]
pub fn calculate_deltas_i(
    problem: &QuboProblem,
    solution: &QuboSolution,
    old_deltas: &[QuboType],
    i: usize,
) -> Vec<QuboType> {
    old_deltas
        .iter()
        .cloned()
        .enumerate()
        .take(i)
        .map(|(j, d_j)| problem.flip_j_and_delta_evaluate_k(solution, d_j, i - 1, j))
        .collect()
}

/// This operation is `O(n 2^n)`
pub fn exhaustive_search_helper(
    problem: &QuboProblem,
    solution: QuboSolution,
    deltas: Vec<QuboType>,
    curr_eval: QuboType,
    i: usize
) -> Vec<(QuboSolution, QuboType)> {
    if i == 0 {
        return vec![(solution, curr_eval)];
    }
    let solution_i = solution.flip(i - 1);
    let eval_i = curr_eval + deltas[i - 1];

    // Update deltas
    let new_deltas = calculate_deltas_i(problem, &solution, &deltas, i);

    let mut left_arr = exhaustive_search_helper(problem, solution, deltas, curr_eval, i - 1);
    let mut right_arr = exhaustive_search_helper(problem, solution_i, new_deltas, eval_i, i - 1);

    left_arr.append(&mut right_arr);
    
    left_arr
}

impl Solver<QuboProblem> for ExhaustiveSearch {
    fn solve(&mut self, qubo_problem: &QuboProblem, mut logger: Option<impl DataRecorder>) -> QuboSolution {
        const BIGGEST_REASONABLE_SEARCH_SIZE: usize = 32;

        let start_solution = QuboSolution(DVector::zeros(qubo_problem.get_size()));

        let delta_j_precalcs: Vec<QuboType> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&start_solution, i))
            .collect();

        if log_enabled!(Warn) && qubo_problem.get_size() > BIGGEST_REASONABLE_SEARCH_SIZE {
            warn!("Exhaustive Searches greater than {BIGGEST_REASONABLE_SEARCH_SIZE} can take extremely long amounts of time! (This algorithm runs in exponential time, but it is provably optimal!)")
        }

        debug!(
            "Starting search of tree of size 2^{}",
            qubo_problem.get_size()
        );

        let solution_list =
            // exhaustive_search_helper(&qubo_problem, start_solution, 0, qubo_problem.get_size());
            exhaustive_search_helper(qubo_problem, start_solution, delta_j_precalcs, 0, qubo_problem.get_size());
        
        if let Some(dr) = &mut logger {
            solution_list.iter().map(|(solution, value)| ExhaustiveSearchRecord {
                solution: format!("{}", solution),
                value: *value
            }).for_each(|x| { dr.add_record(x); });
        }
        
        let (min_solution, min_eval) = solution_list.into_iter().min_by_key(
            |(QuboSolution(solution_vector), eval)| (*eval, -solution_vector.sum()),
        ).unwrap();

        debug!(
            "Produced a provably optimal min evaluation {} with solution: {}",
            min_eval,
            min_solution.0.transpose()
        );

        min_solution
    }
}

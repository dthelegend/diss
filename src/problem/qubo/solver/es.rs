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

// This operation is O(n 2^n) worst-case, O(log n 2^n) average-case
fn exhaustive_search_helper(
    problem: &QuboProblem,
    solution: QuboSolution,
    current_evaluation: QuboType,
    k: usize,
) -> (QuboSolution, QuboType) {
    if k == 0 {
        (solution, current_evaluation)
    } else {
        // Flip k - 1
        let delta_k = problem.delta_evaluate_k(&solution, k - 1);

        // Search left without flip
        let left_min =
            exhaustive_search_helper(problem, solution.clone(), current_evaluation, k - 1);
        // Search right with flip
        let right_min =
            exhaustive_search_helper(problem, solution.flip(k - 1), current_evaluation + delta_k, k - 1);
        // let right_min= {
        //     let (temp_solution, temp_delta_eval) = exhaustive_search_helper_2(
        //         problem,
        //         &solution,
        //         delta_k,
        //         k - 1,
        //         k - 1,
        //     );
        // 
        //     (temp_solution, temp_delta_eval + current_evaluation)
        // };

        min_by_key(left_min, right_min, |(_, eval)| *eval)
    }
}

// TODO get this working!
// fn exhaustive_search_helper_2(
//     problem: &QuboProblem,
//     solution: &QuboSolution,
//     delta_j: QuboType,
//     j: usize,
//     k: usize,
// ) -> (QuboSolution, QuboType) {
//     let solution_j = solution.flip(j);
//     
//     if k == 0 {
//         (solution_j, delta_j)
//     } else {
//         // Flip j - 1
//         let d_kj_minus_d_j = problem.flip_j_and_delta_evaluate_k(solution, delta_j, j,k - 1);
//         
//         // Search left without flip
//         let left_min = exhaustive_search_helper_2(problem, solution, delta_j, j, k - 1);
//         
//         // Search right with flip
//         let right_min = exhaustive_search_helper_2(problem, &solution.flip(k), d_kj_minus_d_j, k - 1, k - 1);
//         
//         min_by_key(left_min, right_min, |(_, eval)| *eval)
//     }
// }

impl QuboSolver for ExhaustiveSearch {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        const BIGGEST_REASONABLE_SEARCH_SIZE: usize = 32;

        let start_solution = QuboSolution(DVector::zeros(qubo_problem.get_size()));

        if log_enabled!(Warn) && qubo_problem.get_size() > BIGGEST_REASONABLE_SEARCH_SIZE {
            warn!("Exhaustive Searches greater than {BIGGEST_REASONABLE_SEARCH_SIZE} can take extremely long amounts of time!")
        }

        debug!(
            "Starting search in tree of size 2^{}",
            qubo_problem.get_size()
        );

        let (min_solution, min_eval) =
            exhaustive_search_helper(&qubo_problem, start_solution, 0, qubo_problem.get_size());

        debug!(
            "Produced a provably optimal min evaluation {} with solution: {}",
            min_eval,
            min_solution.0.transpose()
        );

        min_solution
    }
}

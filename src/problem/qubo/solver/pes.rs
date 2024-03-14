use log::{debug, log_enabled, warn};
use log::Level::Warn;
use nalgebra::DVector;
use rayon::prelude::*;
use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::problem::qubo::solver::QuboSolver;
use crate::problem::qubo::solver::es::exhaustive_search_helper;

#[link(name = "pes")]
extern "C" {
    fn run_pes_solver() -> i32;
}

pub struct ParallelExhaustiveSearch {
    pub beta: usize,
}

impl ParallelExhaustiveSearch {
    pub fn new(beta: usize ) -> Self {
        Self {beta}
    }
}

// Generate all bit strings and deltas for computation. Produces an array of size 2^n
// Can technically generate all solutions for a problem
fn generate_prefixes(qubo_problem: &QuboProblem, solution_list: Vec<(QuboSolution, Vec<QuboType>, QuboType)>, min_i : usize, i : usize) -> Vec<(QuboSolution, Vec<QuboType>, QuboType)> {
    if i <= min_i {
        return solution_list;
    }

    let new_solutions = solution_list
        .into_par_iter()
        .flat_map(|(solution, deltas, eval)| {
            let deltas_i: Vec<_> = deltas
                .iter()
                .cloned()
                .enumerate()
                .take(i)
                .map(|(j, d_j)| qubo_problem.flip_j_and_delta_evaluate_k(&solution, d_j, i - 1, j))
                .collect();
    
            let eval_i = eval + deltas[i - 1];
            let solution_i = solution.flip(i - 1);
    
            [(solution, deltas, eval), (solution_i, deltas_i, eval_i)]
        })
        .collect();
    
    generate_prefixes(qubo_problem, new_solutions, min_i, i - 1)
}

impl QuboSolver for ParallelExhaustiveSearch {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        // TODO infer this from system available parallelism
        const BIGGEST_REASONABLE_SEARCH_SIZE: usize = 32 * 2usize.pow(4);

        if log_enabled!(Warn) && qubo_problem.get_size() > BIGGEST_REASONABLE_SEARCH_SIZE {
            warn!("Exhaustive Searches greater than {BIGGEST_REASONABLE_SEARCH_SIZE} can take extremely long amounts of time! (This algorithm runs in exponential time, but it is provably optimal!)")
        }

        let start_solution = QuboSolution(DVector::zeros(qubo_problem.get_size()));
        let delta_j_precalcs: Vec<QuboType> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&start_solution, i))
            .collect();

        let mut solution_list = vec![(start_solution, delta_j_precalcs, 0)];
        let sub_tree_size = qubo_problem.get_size() - self.beta;
        solution_list = generate_prefixes(&qubo_problem, solution_list, sub_tree_size + 1, qubo_problem.get_size());

        debug!(
            "Starting parallel search across {} processors of tree of size 2^{}",
            solution_list.len(),
            sub_tree_size
        );

        let (min_solution, min_eval) = solution_list.into_par_iter().map(|(solution, deltas, eval)| {
            exhaustive_search_helper(&qubo_problem, solution, deltas, eval, sub_tree_size)
        }).min_by_key(|(_, eval)| *eval).unwrap();

        debug!(
            "Produced a provably optimal min evaluation {} with solution: {}",
            min_eval,
            min_solution.0.transpose()
        );

        min_solution
    }
}
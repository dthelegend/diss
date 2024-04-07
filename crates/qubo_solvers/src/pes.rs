use crate::es::{calculate_deltas_i, exhaustive_search_helper};
use common::Solver;
use log::Level::Warn;
use log::{debug, log_enabled, warn};
use nalgebra::DVector;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use rayon::prelude::*;
use std::usize;
use common::data_recorder::DataRecorder;

pub struct ParallelExhaustiveSearch {
    beta: usize,
}

impl ParallelExhaustiveSearch {
    pub fn new(beta: usize) -> Self {
        Self { beta }
    }
}

// Generate all bit strings and deltas for computation. Produces an array of size 2^n
// Can technically generate all solutions for a problem
// This operation runs in O(n^3) time sequentially, but it is technically O(n^2) across 2^n processors
// Think about it this way: this code runs an O(n) operation, across (2^n) processors,O(n - alpha) times
fn generate_prefixes(
    problem: &QuboProblem,
    solution_list: Vec<(QuboSolution, Vec<QuboType>, QuboType)>,
    min_i: usize,
    i: usize,
) -> Vec<(QuboSolution, Vec<QuboType>, QuboType)> {
    if i <= min_i {
        return solution_list;
    }

    let new_solutions = solution_list
        .into_par_iter()
        .flat_map(|(solution, mut deltas, eval)| {
            let deltas_i: Vec<_> = calculate_deltas_i(problem, &solution, &deltas, i);
            let eval_i = eval + deltas[i - 1];
            let solution_i = solution.flip(i - 1);

            deltas.truncate(deltas_i.len());

            [(solution, deltas, eval), (solution_i, deltas_i, eval_i)]
        })
        .collect();

    generate_prefixes(problem, new_solutions, min_i, i - 1)
}

impl Solver<QuboProblem> for ParallelExhaustiveSearch {
    fn solve(&mut self, qubo_problem: &QuboProblem, _: Option<impl DataRecorder>) -> QuboSolution {
        const BIGGEST_REASONABLE_SEARCH_SIZE: usize = 32;

        if log_enabled!(Warn)
            && qubo_problem.get_size()
                > BIGGEST_REASONABLE_SEARCH_SIZE
                    * (usize::BITS
                        - std::thread::available_parallelism()
                            .unwrap()
                            .get()
                            .leading_zeros()) as usize
        {
            warn!("Exhaustive Searches greater than {BIGGEST_REASONABLE_SEARCH_SIZE} can take extremely long amounts of time! (This algorithm runs in exponential time, but it is provably optimal!)")
        }

        let start_solution = QuboSolution(DVector::zeros(qubo_problem.get_size()));
        let delta_j_precalcs: Vec<QuboType> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&start_solution, i))
            .collect();

        let mut solution_list = vec![(start_solution, delta_j_precalcs, 0)];

        let sub_tree_size = qubo_problem.get_size() + 1 - self.beta;
        solution_list = generate_prefixes(
            qubo_problem,
            solution_list,
            sub_tree_size,
            qubo_problem.get_size(),
        );

        debug!(
            "Starting parallel search across {} processors of tree of size 2^{}",
            solution_list.len(),
            sub_tree_size
        );

        let (min_solution, min_eval) = solution_list
            .into_par_iter()
            .map(|(solution, deltas, eval)| {
                exhaustive_search_helper(qubo_problem, solution, deltas, eval, sub_tree_size)
            })
            .min_by_key(|(QuboSolution(solution), eval)| (*eval, -solution.sum()))
            .unwrap();

        debug!(
            "Produced a provably optimal min evaluation {} with solution: {}",
            min_eval,
            min_solution.0.transpose()
        );

        min_solution
    }
}

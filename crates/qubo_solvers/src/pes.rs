use std::cmp::min;
use std::usize;
use log::Level::Warn;
use log::{debug, log_enabled, warn};
use nalgebra::DVector;
use rayon::prelude::*;
use common::Solver;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use crate::es::{calculate_deltas_i, exhaustive_search_helper};
use crate::pes::pes_gpu::gpu_search_helper;

mod pes_gpu {
    use std::ffi::c_int;

    use nalgebra::DVector;
    use qubo_problem::{QuboProblem, QuboSolution, QuboType};

    #[link(name = "kernels")]
    extern "C" {
        fn run_pes_solver(
            block_size: c_int,
            problem_size: usize,
            qubo_problem: *const QuboType,
            best_solution: *mut QuboType,
            best_evaluatation: *mut QuboType,
            deltas: *const QuboType,
            solution_list: *const QuboType,
            eval_list: *const QuboType,
            i: usize);
    }

    pub fn gpu_search_helper(num_blocks: i32, problem: &QuboProblem, solution_list: Vec<(QuboSolution, Vec<QuboType>, QuboType)>, i: usize) -> (QuboSolution, QuboType) {
        let mut solution_vector = DVector::zeros(problem.get_size());
        let mut best_eval: QuboType = 0;

        let dense_problem = problem.get_dense();

        let (solutions_flat, deltas_flat, evals_flat) = solution_list.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut sf, mut df, mut ef), (s, mut d, e)| {
                sf.extend_from_slice(s.0.as_slice());
                d.resize(problem.get_size(), 0);
                df.append(&mut d);
                ef.push(e);

                (sf, df, ef)
            });
        
        unsafe {
            run_pes_solver(
                num_blocks,
                problem.get_size(),
                dense_problem.as_ptr(),
                solution_vector.as_mut_ptr(),
                &mut best_eval,
                deltas_flat.as_ptr(),
                solutions_flat.as_ptr(),
                evals_flat.as_ptr(),
                i);
        }

        (QuboSolution(solution_vector), best_eval)
    }
}

pub struct ParallelExhaustiveSearch {
    beta: usize,
    use_cuda: bool, 
}

impl ParallelExhaustiveSearch {
    pub fn new(beta: usize) -> Self {
        Self { beta, use_cuda: false }
    }

    pub fn with_cuda(beta: usize) -> Self {
        Self { beta, use_cuda: true }
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
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        const BIGGEST_REASONABLE_SEARCH_SIZE: usize = 32;
        const MAX_CUDA_BLOCK_SIZE: usize = 512;

        if log_enabled!(Warn) && qubo_problem.get_size() > BIGGEST_REASONABLE_SEARCH_SIZE * (usize::BITS - std::thread::available_parallelism().unwrap().get().leading_zeros()) as usize{
            warn!("Exhaustive Searches greater than {BIGGEST_REASONABLE_SEARCH_SIZE} can take extremely long amounts of time! (This algorithm runs in exponential time, but it is provably optimal!)")
        }

        let start_solution = QuboSolution(DVector::zeros(qubo_problem.get_size()));
        let delta_j_precalcs: Vec<QuboType> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&start_solution, i))
            .collect();

        let mut solution_list = vec![(start_solution, delta_j_precalcs, 0)];

        let sub_tree_size = qubo_problem.get_size() - self.beta;
        solution_list = generate_prefixes(
            &qubo_problem,
            solution_list,
            sub_tree_size + 1,
            qubo_problem.get_size(),
        );

        let (min_solution, min_eval) = if !self.use_cuda {
            debug!(
                "Starting parallel search across {} processors of tree of size 2^{}",
                solution_list.len(),
                sub_tree_size
            );

            solution_list
                .into_par_iter()
                .map(|(solution, deltas, eval)| {
                    exhaustive_search_helper(&qubo_problem, solution, deltas, eval, sub_tree_size)
                })
                .min_by_key(|(_, eval)| *eval)
                .unwrap()
        } else {
            debug!(
                "Starting parallel search across {} kernels of tree of size 2^{}",
                solution_list.len(),
                sub_tree_size
            );

            gpu_search_helper(
                min(MAX_CUDA_BLOCK_SIZE, solution_list.len()) as i32,
                &qubo_problem,
                solution_list,
                sub_tree_size)
        };

        debug!(
            "Produced a provably optimal min evaluation {} with solution: {}",
            min_eval,
            min_solution.0.transpose()
        );

        min_solution
    }
}

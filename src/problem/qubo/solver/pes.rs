use crate::problem::qubo::solver::QuboSolver;
use crate::problem::qubo::{QuboProblem, QuboSolution};

#[link(name = "pes")]
extern "C" {
    fn run_pes_solver() -> i32;
}

pub struct ParallelExhaustiveSearch {}

impl ParallelExhaustiveSearch {
    pub fn new() -> Self {
        ParallelExhaustiveSearch {}
    }
}

impl QuboSolver for ParallelExhaustiveSearch {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        // qubo_problem
        let result = unsafe {
            run_pes_solver()
        };

        println!("Status code: {}", result);

        todo!()
    }
}

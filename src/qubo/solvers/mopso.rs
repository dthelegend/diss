use crate::core::Solver;
use crate::qubo::{QuboProblem, QuboSolution};

extern "C" {
    fn run_mopso_solver();
}

pub struct Mopso;

impl Mopso {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver<QuboProblem> for Mopso {
    fn solve(&mut self, problem: &QuboProblem) -> QuboSolution {
        unsafe {
            run_mopso_solver();
        }

        unimplemented!()
    }
}
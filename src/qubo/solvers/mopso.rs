use crate::core::Solver;
use crate::qubo::{QuboProblem, QuboSolution};

pub struct Mopso;

impl Mopso {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver<QuboProblem> for Mopso {
    fn solve(&mut self, _problem: &QuboProblem) -> QuboSolution {
        unimplemented!()
    }
}
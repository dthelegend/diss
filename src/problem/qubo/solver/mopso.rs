use crate::problem::qubo::solver::QuboSolver;
use crate::problem::qubo::{QuboProblem, QuboSolution};

pub struct Mopso {}

impl Mopso {
    pub fn new() -> Self {
        Mopso {}
    }
}

impl QuboSolver for Mopso {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        todo!()
    }
}

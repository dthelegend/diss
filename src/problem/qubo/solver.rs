mod sa;

use crate::problem::qubo::{QuboProblem, QuboSolution};

pub trait QuboSolver {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution;
}

pub use sa::SimulatedAnnealer;
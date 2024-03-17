mod es;
mod pes;
mod sa;

use crate::problem::qubo::{QuboProblem, QuboSolution};

pub trait QuboSolver {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution;
}

pub use es::ExhaustiveSearch;
pub use pes::ParallelExhaustiveSearch;
pub use sa::SimulatedAnnealer;

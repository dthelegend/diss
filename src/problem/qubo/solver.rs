mod es;
mod mopso;
mod pes;
mod sa;

use crate::problem::qubo::{QuboProblem, QuboSolution};

pub trait QuboSolver {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution;
}

pub use es::ExhaustiveSearch;
pub use mopso::Mopso;
pub use pes::ParallelExhaustiveSearch;
pub use sa::SimulatedAnnealer;

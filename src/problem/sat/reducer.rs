mod chancellor;
mod choi;

use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::sat::{KSatProblem, SatSolution};

pub use chancellor::Chancellor;
pub use choi::Choi;

pub trait QuboToSatReduction {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self);

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution;
}

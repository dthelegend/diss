use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution};

pub struct Chancellor {}

impl QuboToSatReduction for Chancellor {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        todo!()
    }

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution {
        todo!()
    }
}

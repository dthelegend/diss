use nalgebra_sparse::CooMatrix;
use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution};

pub struct Choi {}

impl QuboToSatReduction for Choi {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        // let matrix = CooMatrix::new();

        todo!()
    }

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution {
        todo!()
    }
}
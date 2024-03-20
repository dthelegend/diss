use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution};

pub struct Nusslein23 {}

impl QuboToSatReduction for Nusslein23 {
    fn reduce(
        &KSatProblem {
            nb_vars,
            ref clause_list,
        }: &KSatProblem,
    ) -> (QuboProblem, Self) {
        // for each var_i, i = 2 * var_i maps to var_i
        // for each var_i, i = 2* var_i + 1 maps to ¬var_i
        for clause in clause_list {}
        // K auxiliary vars are then created for

        todo!()
    }

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution {
        todo!()
    }
}

use super::{ReducibleProblem, qubo::{QuboSolution, QuboProblem}, sat::{KSatProblem, KSatSolution}};

pub enum SatToQuboReduction {
    Choi,
    Chancellor,
    // TODO: Other Reductions
}

impl ReducibleProblem<SatToQuboReduction, KSatSolution, bool, QuboSolution, i32> for KSatProblem {
    fn reduce(&self, reduction: SatToQuboReduction) -> Box<dyn super::Problem<QuboSolution, i32>> {
        match reduction {
            SatToQuboReduction::Choi => todo!(),
            SatToQuboReduction::Chancellor => todo!(),
        }
    }

    fn convert_solution(&self, solution : QuboSolution) -> KSatSolution {
        todo!()
    }
}
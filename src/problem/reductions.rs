use super::{ReducibleProblem, qubo::{QuboSolution, QuboProblem}, sat::{KSatProblem, KSatSolution}};

#[derive(Clone, Copy)]
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

    fn convert_solution(&self, reduction: SatToQuboReduction, solution : QuboSolution) -> KSatSolution {
        match reduction {
            SatToQuboReduction::Choi => todo!(),
            SatToQuboReduction::Chancellor => todo!(),
        }
    }
}
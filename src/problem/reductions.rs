use super::{ReducibleProblem, qubo::{QuboSolution, QuboProblem}, sat::{KSatProblem, KSatSolution}};

#[derive(Clone, Copy, Default)]
pub enum SatToQuboReduction {
    // An optimised reduction of K-SAT to 3-SAT to MIS to QUBO
    // Not expecting this to run well :/
    Choi,
    // An optimised reduction from 3 SAT to Max-2-SAT to QUBO
    // Also not expecting this to run well :/
    Novel,
    // The current state-of-the-art reduction
    #[default]
    Chancellor,
    // A reduction that scales well for sub-quadratic QUBO formulations
    // You cannot tell looking at a problem whether |E| = O(k|V|) or |E| = O(|V|*|V|)
    Nuesslein2022,
    // A reduction that scales better than Chancellor, but the paper on it is still in preprint
    Nuesslein2023,
}

impl ReducibleProblem<SatToQuboReduction, KSatSolution, bool, QuboSolution, i32> for KSatProblem {
    fn reduce(&self, reduction: SatToQuboReduction) -> Box<dyn super::Problem<QuboSolution, i32>> {
        match reduction {
            SatToQuboReduction::Choi => todo!(),
            SatToQuboReduction::Chancellor => todo!(),
            SatToQuboReduction::Nuesslein2022 => todo!(),
            SatToQuboReduction::Nuesslein2023 => todo!(),
            SatToQuboReduction::Novel => todo!(),
        }
    }

    fn convert_solution(&self, reduction: SatToQuboReduction, solution : QuboSolution) -> KSatSolution {
        match reduction {
            SatToQuboReduction::Choi => todo!(),
            SatToQuboReduction::Chancellor => todo!(),
            SatToQuboReduction::Nuesslein2022 => todo!(),
            SatToQuboReduction::Nuesslein2023 => todo!(),
            SatToQuboReduction::Novel => todo!(),
        }
    }
}
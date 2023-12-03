use crate::problem::{sat::{SatSolution, ksat::KSatProblem}, qubo::{QuboProblem, QuboSolution}};

use super::{*, sat_to_qubo::ThreeSatToQuboReduction, ksat_to_threesat::KSatToThreeSatReduction};

#[derive(Default)]
pub enum KSatToQuboReduction {
    /// An optimised reduction of K-SAT to 3-SAT to MIS to QUBO <br/>
    /// Not expecting this to run well :/
    Choi,
    /// An optimised reduction from 3 SAT to Max-2-SAT to QUBO <br/>
    /// Also not expecting this to run well :/
    Novel,
    /// The current state-of-the-art reduction
    #[default]
    Chancellor,
    /// A reduction that scales well for sub-quadratic QUBO formulations <br/>
    /// You cannot tell looking at a problem whether |E| = O(k|V|) or |E| = O(|V|*|V|)
    Nuesslein2022,
    /// A reduction that scales better than Chancellor, but the paper on it is still in preprint
    Nuesslein2023,
}

impl Reduction<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboReduction {
    fn reduce_problem(&self, problem: &KSatProblem) -> QuboProblem {
        match self {
            KSatToQuboReduction::Choi => {
                let threesat_problem = KSatToThreeSatReduction.reduce_problem(problem);

                ThreeSatToQuboReduction::Choi.reduce_problem(&threesat_problem)
            },
            KSatToQuboReduction::Novel => todo!(),
            KSatToQuboReduction::Chancellor => todo!(),
            KSatToQuboReduction::Nuesslein2022 => todo!(),
            KSatToQuboReduction::Nuesslein2023 => todo!(),
        }
    }
}

impl SolutionReversibleReduction<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboReduction {
    fn reverse_reduce_solution(&self, problem: &KSatProblem, solution: QuboSolution) -> SatSolution {
        match self {
            KSatToQuboReduction::Choi => {
                let threesat_problem = KSatToThreeSatReduction.reduce_problem(problem);
                
                let threesat_solution = ThreeSatToQuboReduction::Choi.reverse_reduce_solution(&threesat_problem, solution);

                KSatToThreeSatReduction.reverse_reduce_solution(problem, threesat_solution)
            },
            KSatToQuboReduction::Novel => todo!(),
            KSatToQuboReduction::Chancellor => todo!(),
            KSatToQuboReduction::Nuesslein2022 => todo!(),
            KSatToQuboReduction::Nuesslein2023 => todo!(),
        }
    }
}
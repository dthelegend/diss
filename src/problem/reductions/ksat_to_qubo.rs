
use crate::problem::{sat::{SatSolution, ksat::KSatProblem}, qubo::{QuboProblem, QuboSolution}};

use super::{sat_to_qubo::SatToQuboReduction, Reducer, ReducedProblem, ksat_to_threesat::KSatToThreeSatReducer};

#[derive(Clone, Copy, Default)]
pub enum KSatToQuboReducer {
    /// An optimised reduction of K-SAT to 3-SAT to MIS to QUBO
    /// Not expecting this to run well :/
    Choi,
    /// An optimised reduction from 3 SAT to Max-2-SAT to QUBO
    /// Also not expecting this to run well :/
    Novel,
    /// The current state-of-the-art reduction
    #[default]
    Chancellor,
    /// A reduction that scales well for sub-quadratic QUBO formulations
    /// You cannot tell looking at a problem whether |E| = O(k|V|) or |E| = O(|V|*|V|)
    Nuesslein2022,
    /// A reduction that scales better than Chancellor, but the paper on it is still in preprint
    Nuesslein2023,
}

pub struct KSatToQuboReducedProblem<'a> {
    problem: &'a KSatProblem,
    reduced_problem: QuboProblem,
    reduction: KSatToQuboReducer
}

impl Reducer<SatSolution, bool, KSatProblem, QuboSolution, i32, QuboProblem> for KSatToQuboReducer {
    fn reduce(self, problem: &KSatProblem) -> Box<dyn ReducedProblem<SatSolution, bool, KSatProblem, QuboSolution, i32, QuboProblem> + '_> {
        let reduced_problem = match self {
            KSatToQuboReducer::Choi => {
                let threesat_version = KSatToThreeSatReducer.reduce(problem);
                let qubo_version = SatToQuboReduction::Choi.reduce(threesat_version.get_reduced_problem());

                qubo_version.get_reduced_problem()
            },
            KSatToQuboReducer::Novel => todo!(),
            KSatToQuboReducer::Chancellor => todo!(),
            KSatToQuboReducer::Nuesslein2022 => todo!(),
            KSatToQuboReducer::Nuesslein2023 => todo!(),
        };

        Box::new(KSatToQuboReducedProblem {
            problem,
            reduced_problem: *reduced_problem,
            reduction: self
        })
    }
}

impl <'a> ReducedProblem<SatSolution, bool, KSatProblem, QuboSolution, i32, QuboProblem> for KSatToQuboReducedProblem<'a> {
    fn get_reduced_problem(&self) -> &QuboProblem {
        &self.reduced_problem
    }

    fn get_original_problem(&self) -> &KSatProblem {
        &self.problem
    }

    fn convert_solution(&self, solution : QuboSolution) -> SatSolution {
        todo!()
    }
}
use std::iter::zip;
use crate::problem::{Problem, sat::{SatSolution, ksat::KSatProblem, threesat::ThreeSatProblem, SatVariable}, qubo::{QuboProblem, QuboSolution}};

use super::{sat::SatToQuboReduction, Reducer};

#[derive(Clone, Copy, Default)]
pub enum KSatToQuboReducer {
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

pub struct KSatToQuboReducedProblem<'a> {
    problem: &'a KSatProblem,
    reduced_problem: QuboProblem,
    reduction: KSatToQuboReducer
}

impl Reducer<SatSolution, bool, QuboSolution, i32> for KSatToQuboReducer {
    fn reduce(self, problem: &dyn Problem<KSatProblem, bool>) -> Box<dyn super::ReducedProblem<SatSolution, bool, QuboSolution, i32>> {
        let reduced_problem = match self {
            KSatToQuboReducer::Choi => {
                let sat_self = self.reduce(KSatToSatReducer::Standard(self.0));
                let qubo_self = ThreeSatProblem::reduce(sat_self, SatToQuboReduction::Choi);

                Ok(())
            },
            KSatToQuboReducer::Novel => todo!(),
            KSatToQuboReducer::Chancellor => todo!(),
            KSatToQuboReducer::Nuesslein2022 => todo!(),
            KSatToQuboReducer::Nuesslein2023 => todo!(),
        };

        KSatToQuboReducedProblem {
            problem,
            reduced_problem,
            reduction: self
        }
    }
}
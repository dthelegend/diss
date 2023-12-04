use log::debug;

use crate::problem::{sat::{SatSolution, ksat::KSatProblem, threesat::ThreeSatProblem}, qubo::{QuboProblem, QuboSolution}};

use super::{*, sat_to_qubo::ThreeSatToQuboReduction, ksat_to_threesat::KSatToThreeSatReduction};

#[derive(Default)]
pub enum KSatToQuboReduction {
    /// An optimised reduction of K-SAT to 3-SAT to MIS to QUBO <br/>
    /// Not expecting this to run well :/ <br />
    /// Reduction happens in O(n^2)
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

pub enum KSatToQuboSolutionReductionReverser {
    Choi {
        threesat_reverser: Box<dyn SolutionReductionReverser<SatSolution, KSatProblem, SatSolution, ThreeSatProblem>>,
        qubo_reverser: Box<dyn SolutionReductionReverser<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem>>
    },
    Novel,
    Chancellor,
    Nuesslein2022,
    Nuesslein2023,
}

impl Reduction<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboReduction {
    fn reduce_problem(&self, problem: KSatProblem) -> (QuboProblem, Box<dyn SolutionReductionReverser<SatSolution, KSatProblem, QuboSolution, QuboProblem>>) {
        match self {
            KSatToQuboReduction::Choi => {
                let (threesat_problem, threesat_reverser) = KSatToThreeSatReduction.reduce_problem(problem);

                debug!("Choi reduction part 1 produced: {:?}", threesat_problem);

                let (qubo_problem, qubo_reverser) = ThreeSatToQuboReduction::Choi.reduce_problem(threesat_problem);

                debug!("Choi reduction part 2 produced: {}", qubo_problem);

                (qubo_problem, Box::new(KSatToQuboSolutionReductionReverser::Choi { threesat_reverser, qubo_reverser}))
            },
            KSatToQuboReduction::Novel => todo!(),
            KSatToQuboReduction::Chancellor => todo!(),
            KSatToQuboReduction::Nuesslein2022 => todo!(),
            KSatToQuboReduction::Nuesslein2023 => todo!(),
        }
    }
}

impl SolutionReductionReverser<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboSolutionReductionReverser {
    fn reverse_reduce_solution(&self, solution: QuboSolution) -> SatSolution {
        match self {
            KSatToQuboSolutionReductionReverser::Choi {qubo_reverser, threesat_reverser} => {
                let threesat_solution = qubo_reverser.reverse_reduce_solution(solution);

                threesat_reverser.reverse_reduce_solution(threesat_solution)
            },
            KSatToQuboSolutionReductionReverser::Novel => todo!(),
            KSatToQuboSolutionReductionReverser::Chancellor => todo!(),
            KSatToQuboSolutionReductionReverser::Nuesslein2022 => todo!(),
            KSatToQuboSolutionReductionReverser::Nuesslein2023 => todo!(),
        }
    }
}
use std::iter::zip;
use crate::problem::{sat::{threesat::ThreeSatProblem, SatSolution, SatVariable}, qubo::{QuboSolution, QuboProblem, QuboProblemBackend}, Problem};

use super::{Reducer, ReducedProblem};

pub enum SatToQuboReduction {
    Choi
}

pub struct SatToQuboReducedProblem<'a> {
    problem: &'a ThreeSatProblem,
    reduced_problem: QuboProblem,
    reduction: SatToQuboReduction
}

impl Reducer<SatSolution, bool, QuboSolution, i32> for SatToQuboReduction {
    fn reduce(self, problem: &dyn Problem<SatSolution, bool>) -> Box<dyn super::ReducedProblem<SatSolution, bool, QuboSolution, i32>> {
        match self {
            SatToQuboReduction::Choi => {
                let ThreeSatProblem { nbvars: size, clauses: threesatclauses } = self;

                let mut reduced_problem = QuboProblem::new_with_backend(threesatclauses.len() * 3, QuboProblemBackend::SimulatedAnnealing);

                for (i, xn) in threesatclauses.iter().enumerate() {
                    // Initialise the clauses
                    reduced_problem[(i * 3, i * 3)] = -1;
                    reduced_problem[(i * 3 + 1, i * 3 + 1)] = -1;
                    reduced_problem[(i * 3 + 2, i * 3 + 2)] = -1;
                    // Creates a triangle of clauses
                    reduced_problem[(i * 3, i * 3 + 1)] = 3;
                    reduced_problem[(i * 3, i * 3 + 2)] = 3;
                    reduced_problem[(i * 3 + 1, i * 3 + 2)] = 3;

                    for (j, yn) in threesatclauses.iter().enumerate() {
                        // If the clauses are in conflict add an edge
                        if i != j {
                            // Check for conflict between clauses xn and yn
                            let conflict_list = zip(xn.iter().enumerate(), yn.iter().enumerate())
                                .filter(|((_, SatVariable(is_pos_xi, xi)), (_, SatVariable(is_pos_yi, yi)))| yi == xi && (is_pos_xi ^ is_pos_yi))
                                .map(|((i,_), (j,_))| (i,j));
                        
                            for conflict in conflict_list {
                                // Add an edge between nodes i and j
                                reduced_problem[conflict] = 3;
                            }
                        }
                    }
                }

                // NB: For the MIS Problem, the goal is to get an MIS with |V| = threesatclauses.len()

                SatToQuboReducedProblem {problem, reduced_problem, reduction: self }
            }
        }
    }
}

impl <'a> ReducedProblem<SatSolution, bool, QuboSolution, i32> for SatToQuboReducedProblem<'a> {
    fn get_reduced_problem(&self) -> &dyn Problem<QuboSolution, i32> {
        &self.problem
    }

    fn convert_solution(&self, solution : QuboSolution) -> SatSolution {
        match self.reduction {
            SatToQuboReduction::Choi => {
                let QuboSolution(solution) = solution;

                if solution.chunks(3).map(|c| c.iter().any(|f| *f)).filter(|f| *f).count() < solution.len() / 3 {
                    SatSolution::Unsat
                }
                else {
                    SatSolution::Unknown
                }
            }
        }
    }
}
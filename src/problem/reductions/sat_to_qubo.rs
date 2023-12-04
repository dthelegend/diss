use std::iter::zip;
use log::debug;

use crate::problem::{sat::{threesat::ThreeSatProblem, SatSolution, SatVariable}, qubo::{QuboSolution, QuboProblem, QuboProblemBackend}};

use super::*;

pub enum ThreeSatToQuboReduction {
    Choi
}

impl Reduction<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem> for ThreeSatToQuboReduction {
    fn reduce_problem(&self, problem: &ThreeSatProblem) -> QuboProblem {
        match self {
            ThreeSatToQuboReduction::Choi => {
                let ThreeSatProblem( .. , threesatclauses) = problem;

                let mut reduced_problem = QuboProblem::new_with_backend(threesatclauses.len() * 3, QuboProblemBackend::SimulatedAnnealing);

                for (i, xn) in threesatclauses.iter().enumerate() {
                    // Initialise the clauses
                    reduced_problem[(i * 3, i * 3)] = -1;
                    reduced_problem[(i * 3 + 1, i * 3 + 1)] = -1;
                    reduced_problem[(i * 3 + 2, i * 3 + 2)] = -1;
                    // Creates a triangle of clauses
                    reduced_problem[(i * 3, i * 3 + 1)] = 10;
                    reduced_problem[(i * 3, i * 3 + 2)] = 10;
                    reduced_problem[(i * 3 + 1, i * 3 + 2)] = 10;

                    for (j, yn) in threesatclauses.iter().enumerate() {
                        // If the clauses are in conflict add an edge
                        if i != j {
                            // Check for conflict between clauses xn and yn
                            for (c_xi, c_x) in xn.iter().enumerate() {
                                for (c_yj, c_y) in yn.iter().enumerate() {
                                    if c_x.1 == c_y.1 && c_x.0 ^ c_y.0 {
                                        reduced_problem[(3 * i + c_xi, 3 * j + c_yj)] = 10
                                    }
                                }
                            }
                        }
                    }
                }

                // NB: For the MIS Problem, the goal is to get an MIS with |V| = threesatclauses.len()

                println!("{:?}", reduced_problem);

                reduced_problem
            }
        }
    }
}

impl SolutionReversibleReduction<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem> for ThreeSatToQuboReduction {
    fn reverse_reduce_solution(&self, problem: &ThreeSatProblem, solution: QuboSolution) -> SatSolution {
        let ThreeSatProblem( .. , threesatclauses) = problem;
        let QuboSolution( x ) = solution;

        if !x.chunks(3).map(|f| f.iter().any(|f| *f)).all(|f| f) {
            return SatSolution::Unsat;
        }

        let mut output_solution_vector: Vec<bool> = vec![false; problem.0];

        println!("{:?}", problem);

        for SatVariable(is_pos, b) in zip(x.chunks(3), threesatclauses).flat_map(|(b, v)| zip(b,v)).filter(|(b, _)| **b).map(|(_,v)| v) {
            output_solution_vector[*b] = *is_pos;
        }

        SatSolution::Sat(output_solution_vector)
    }
}

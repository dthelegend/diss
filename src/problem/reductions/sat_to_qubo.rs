use std::iter::zip;

use crate::problem::{sat::{threesat::ThreeSatProblem, SatSolution, SatVariable}, qubo::{QuboSolution, QuboProblem, QuboProblemBackend}};

use super::*;

pub enum ThreeSatToQuboReduction {
    /// This reduction is O(n^2). Technically still polynomial time
    Choi
}

pub enum ThreeSatToQuboSolutionReductionReverser {
    Choi(usize, Vec<[SatVariable; 3]>)
}

impl Reduction<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem> for ThreeSatToQuboReduction {
    fn reduce_problem(&self, problem: ThreeSatProblem) -> (QuboProblem, Box<dyn SolutionReductionReverser<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem>>) {
        match self {
            ThreeSatToQuboReduction::Choi => {
                const EDGE_WEIGHT: i32 = 10;
                const VERTEX_WEIGHT: i32 = -1;
                let ThreeSatProblem( size , threesatclauses) = problem;

                let mut reduced_problem = QuboProblem::new_with_backend(threesatclauses.len() * 3, QuboProblemBackend::SimulatedAnnealing);

                for (i, xn) in threesatclauses.iter().enumerate() {
                    // Initialise the clauses
                    reduced_problem[(i * 3, i * 3)] = VERTEX_WEIGHT;
                    reduced_problem[(i * 3 + 1, i * 3 + 1)] = VERTEX_WEIGHT;
                    reduced_problem[(i * 3 + 2, i * 3 + 2)] = VERTEX_WEIGHT;
                    // Creates a triangle of clauses
                    reduced_problem[(i * 3, i * 3 + 1)] = EDGE_WEIGHT;
                    reduced_problem[(i * 3, i * 3 + 2)] = EDGE_WEIGHT;
                    reduced_problem[(i * 3 + 1, i * 3 + 2)] = EDGE_WEIGHT;

                    // TODO: Potential for optimisation
                    for (j, yn) in threesatclauses.iter().enumerate() {
                        if i != j {
                            // Check for conflict between clauses xn and yn
                            for (c_xi, c_x) in xn.iter().enumerate() {
                                for (c_yj, c_y) in yn.iter().enumerate() {
                                    if c_x.1 == c_y.1 && c_x.0 ^ c_y.0 {
                                        reduced_problem[(3 * i + c_xi, 3 * j + c_yj)] = EDGE_WEIGHT;
                                    }
                                }
                            }
                        }
                    }
                }

                (reduced_problem, Box::new(ThreeSatToQuboSolutionReductionReverser::Choi(size, threesatclauses)))
            }
        }
    }
}

impl SolutionReductionReverser<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem> for ThreeSatToQuboSolutionReductionReverser {
    fn reverse_reduce_solution(&self, solution: QuboSolution) -> SatSolution {
        match self {
            ThreeSatToQuboSolutionReductionReverser::Choi(nbvars, threesatclauses) => {
                let QuboSolution( x ) = solution;
        
                // NB: For the MIS Problem, the goal is to get an MIS with |V| = threesatclauses.len()
                
                // There must be exactly one true statement per clause for our solution to be SAT
                if !x.chunks(3).map(|f| f.iter().filter(|f| **f).count()).all(|f| f == 1) {
                    // Unknown as without an exhaustive search of the solution space a couple more times to be sure we cannot know if
                    // The problem is truly UNSAT and SA as implemented doesn't produce a way to track this
                    return SatSolution::Unknown;
                }
        
                let mut output_solution_vector: Vec<bool> = vec![false; *nbvars];
        
                for SatVariable(is_pos, b) in zip(x.chunks(3), threesatclauses).flat_map(|(b, v)| zip(b,v)).filter(|(b, _)| **b).map(|(_,v)| v) {
                    output_solution_vector[*b] = *is_pos;
                }
        
                SatSolution::Sat(output_solution_vector)
            },
        }
    }
}

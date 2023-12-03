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

        let mut output_solution_vector: Vec<Option<bool>> = vec![None; problem.0];

        println!("{:?}", problem);

        for (is_pos_in_model, SatVariable(is_pos, b)) in zip(x.chunks(3), threesatclauses).flat_map(|(b, v)| zip(b,v)) {
            let evaluated_value = !(is_pos_in_model ^ is_pos);
            if let Some(existing_value) = output_solution_vector[*b] {
                // Model Conflicts with itself. Solution is therefore not optimal
                if existing_value != evaluated_value {
                    debug!("Model Conflicts with itself! Solution is therefore not optimal! {}{b} != {}{b}", if existing_value { "¬" } else { "" }, if evaluated_value { "¬" } else { "" });
                    return SatSolution::Unknown;
                }
            }
            else {
                output_solution_vector[*b] = Some(evaluated_value)
            }
        }

        SatSolution::Sat(output_solution_vector.iter().map(|x| x.expect("All variables should be modelled!")).collect())
    }
}

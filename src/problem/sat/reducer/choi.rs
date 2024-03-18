use log::{trace, warn};
use nalgebra::DVector;
use nalgebra_sparse::{CooMatrix, CsrMatrix};

use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution, SatVariable};

// Choi scales directly in the number of clause variables and therefore the size of the problem
pub struct Choi {
    map: Vec<(Vec<usize>, Vec<usize>)>,
}

impl QuboToSatReduction for Choi {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        const VERTEX_WEIGHT: isize = -1;
        const EDGE_PENALTY: isize = 0;
        const EDGE_WEIGHT: isize = -(2 * VERTEX_WEIGHT) + EDGE_PENALTY;

        let total_number_of_clause_vars = sat_problem.clause_list.iter().map(|x| x.len()).sum();
        let mut matrix_constructor =
            CooMatrix::new(total_number_of_clause_vars, total_number_of_clause_vars);

        let mut map = vec![(Vec::new(), Vec::new()); sat_problem.nb_vars];

        let mut clause_counter = 0;
        for clause in &sat_problem.clause_list {
            let clause_len = clause.len();

            for (i, var_i) in clause.iter().cloned().enumerate() {
                // Construct a node weight here
                let clause_reference_i = clause_counter + i;

                matrix_constructor.push(clause_reference_i, clause_reference_i, VERTEX_WEIGHT);

                let SatVariable(is_true, number) = var_i;
                let (true_reference_list, false_reference_list) = &mut map[number];
                if is_true {
                    true_reference_list
                } else {
                    false_reference_list
                }
                .push(clause_reference_i);

                for j in (i + 1)..clause_len {
                    // Create a connection to all other nodes
                    matrix_constructor.push(clause_reference_i, clause_counter + j, EDGE_WEIGHT);
                }
            }

            clause_counter += clause_len;
        }

        for (true_reference_list, false_reference_list) in &map {
            for true_reference in true_reference_list {
                for false_reference in false_reference_list {
                    let mut i = *true_reference;
                    let mut j = *false_reference;
                    if i > j {
                        (i, j) = (j, i)
                    }
                    matrix_constructor.push(i, j, EDGE_WEIGHT);
                }
            }
        }

        let q_matrix = CsrMatrix::from(&matrix_constructor);

        (
            QuboProblem::try_from_q_matrix(q_matrix)
                .expect("Q Matrix has been explicitly constructed of the correct size"),
            Choi { map },
        )
    }

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution {
        let QuboSolution(solution_vector) = qubo_solution;

        let mut output_vector: Vec<bool> = Vec::with_capacity(self.map.len());
        for i in 0..self.map.len() {
            let (true_reference_list, false_reference_list) = &self.map[i];

            let v_i = if true_reference_list.is_empty() {
                // Only check for false references if there are no true references
                !false_reference_list
                    .iter()
                    .map(|x| solution_vector[*x])
                    .any(|x| x == 1)
            } else if false_reference_list.is_empty() {
                // Only check for true references if there are no false references
                true_reference_list
                    .iter()
                    .map(|x| solution_vector[*x])
                    .any(|x| x == 1)
            } else {
                // There is a positive assertion that x is true
                let is_true = !true_reference_list
                    .iter()
                    .map(|x| solution_vector[*x])
                    .any(|x| x == 0);
                // There is a positive assertion that x is false
                let is_false = !false_reference_list
                    .iter()
                    .map(|x| solution_vector[*x])
                    .any(|x| x == 0);

                if !(is_true || is_false) {
                    // This variable is never chosen as the true variable in any clause, and therefore must be assumed false
                    false
                } else if is_true && is_false {
                    warn!("Conflict found when up-modelling for Variable {i}!");
                    trace!("Model for {i} is (is_true: {true_reference_list:?} / {is_true}) (is_false: {false_reference_list:?} / {is_false})");

                    return SatSolution::Unsat;
                } else {
                    is_true && !is_false
                }
            };

            output_vector.push(v_i);
        }

        SatSolution::Sat(DVector::from_vec(output_vector))
    }
}

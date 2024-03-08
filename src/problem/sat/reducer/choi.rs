use log::Level::Trace;
use log::{debug, log_enabled};
use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution, SatVariable};

pub struct Choi {
    map: Vec<(Vec<usize>, Vec<usize>)>,
}

impl QuboToSatReduction for Choi {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        const VERTEX_WEIGHT: isize = -1;
        const EDGE_PENALTY: isize = 10;
        const EDGE_WEIGHT: isize = -(2 * VERTEX_WEIGHT) + EDGE_PENALTY;

        let total_number_of_clause_vars = sat_problem.clause_list.iter().map(|x| x.len()).sum();
        let mut matrix_constructor =
            CooMatrix::new(total_number_of_clause_vars, total_number_of_clause_vars);

        let mut map = vec![(Vec::new(), Vec::new()); sat_problem.nb_vars];

        let mut clause_counter = 0;
        for clause in &sat_problem.clause_list {
            let clause_len = clause.len();

            for i in 0..clause_len {
                // Construct a node weight here
                let clause_reference_i = clause_counter + i;

                matrix_constructor.push(clause_reference_i, clause_reference_i, VERTEX_WEIGHT);

                let SatVariable(is_true, number) = clause[i];
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

            clause_counter += clause_len;
        }

        let q_matrix = CsrMatrix::from(&matrix_constructor);

        if log_enabled!(Trace) {
            let q_matrix_for_printing =
                q_matrix.clone() * DMatrix::identity(q_matrix.ncols(), q_matrix.ncols());
            debug!("Choi reduction q-matrix {}", q_matrix_for_printing);
        }

        (
            QuboProblem::try_from_q_matrix(q_matrix)
                .expect("Q Matrix has been explicitly constructed of the correct size"),
            Choi { map },
        )
    }

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution {
        let QuboSolution(solution_vector) = qubo_solution;

        SatSolution::Sat(DVector::from_iterator(
            self.map.len(),
            self.map
                .iter()
                .map(|(true_reference_list, false_reference_list)| {
                    // There is a positive assertion that x is true
                    let is_true = true_reference_list
                        .iter()
                        .map(|x| solution_vector[*x])
                        .any(|x| x == 1);
                    // There is a positive assertion that x is false
                    let is_false = false_reference_list
                        .iter()
                        .map(|x| solution_vector[*x])
                        .any(|x| x == 1);

                    if is_true {
                        1
                    } else if is_false {
                        0
                    } else {
                        // TODO This is an error that is currently not caught
                        // Assume false due to conflict
                        0
                    }
                }),
        ))
    }
}

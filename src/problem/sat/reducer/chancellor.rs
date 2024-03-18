use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution, SatVariable};
use nalgebra::DVector;
use nalgebra_sparse::{CooMatrix, CsrMatrix};

pub struct Chancellor(usize);

impl QuboToSatReduction for Chancellor {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        const J: QuboType = 100;
        const J_A: QuboType = J;
        const H: QuboType = -J_A;
        const G: QuboType = 5; // NB G <<

        let &KSatProblem {
            nb_vars,
            ref clause_list,
        } = sat_problem;

        let num_ancillae: usize = clause_list.iter().map(|x| x.len()).sum();
        let total_clauses = nb_vars + num_ancillae;

        let mut q_matrix = CooMatrix::new(total_clauses, total_clauses);
        let mut new_var_counter = 0;

        for clause in clause_list.iter() {
            for (i, SatVariable(is_true_i, var_i)) in clause.iter().cloned().enumerate() {
                let c_i = 2 * (is_true_i as QuboType) - 1;

                for (j, SatVariable(is_true_j, var_j)) in clause.iter().take(i).cloned().enumerate()
                {
                    // First section
                    let c_j = 2 * (is_true_j as QuboType) - 1;
                    if var_i < var_j {
                        q_matrix.push(var_i, var_j, J * c_i * c_j);
                    } else {
                        q_matrix.push(var_j, var_i, J * c_i * c_j);
                    }

                    // Auxiliary variable section one
                    let j_a = nb_vars + new_var_counter + j;
                    q_matrix.push(var_i, j_a, J * c_i);
                }

                // Second Section
                q_matrix.push(var_i, var_i, H * c_i);

                // Auxiliary Variable section one
                let i_a = nb_vars + new_var_counter + i;

                let q_i = if i == 0 { G / 2 } else { 0 };

                let h_a_i = -J_A * ((2 * i as QuboType) - clause.len() as QuboType) + q_i;

                q_matrix.push(i_a, i_a, h_a_i);
            }

            new_var_counter += clause.len();
        }

        assert_eq!(num_ancillae, new_var_counter);
        (
            QuboProblem::try_from_q_matrix(CsrMatrix::from(&q_matrix)).unwrap(),
            Self(nb_vars),
        )
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        SatSolution::Sat(DVector::from_fn(self.0, |i, _| solution_vector[i] != 0))
    }
}

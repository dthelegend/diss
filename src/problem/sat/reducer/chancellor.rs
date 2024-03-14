use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution};
use log::debug;
use nalgebra::{DMatrix, DVector, Matrix};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

pub struct Chancellor(usize);

impl QuboToSatReduction for Chancellor {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        let KSatProblem {
            nb_vars,
            clause_list,
        } = sat_problem;

        let num_ancillae: usize = clause_list.iter().map(|x| x.len()).sum();

        debug!("Chancellor reduction requires {} ancillae", num_ancillae);

        let mut q_matrix: DMatrix<QuboType> =
            DMatrix::zeros(nb_vars + num_ancillae, nb_vars + num_ancillae);

        // Coupling Strength in the ising model
        const J: QuboType = 2;
        // Site Strength in the ising model
        const H: QuboType = -J;
        // Not sure?
        const G: QuboType = 1;

        let mut new_var_counter = 0;
        for clause in clause_list {
            // \sum^k_{i=1}
            for (i_index, i_var) in clause.iter().enumerate() {
                // \sum^{i-1}_{j=1}
                for (j_index, j_var) in clause[0..i_index].iter().enumerate() {
                    // J c(i) c(j)\sigma^z_i\sigma^z_j
                    let sign = if i_var.0 == j_var.0 { 1 } else { -1 };
                    if i_var.1 < j_var.1 {
                        q_matrix[(i_var.1, j_var.1)] += 4 * sign * J;
                    } else {
                        q_matrix[(j_var.1, i_var.1)] += 4 * sign * J;
                    }

                    // J^a c(i)\sigma^z_i\sigma^z_{j,a}
                    let aux_variable = nb_vars + new_var_counter + j_index;
                    q_matrix[(i_var.1, aux_variable)] += 4 * J * (if i_var.0 { 1 } else { -1 });
                }

                // h c(i)\sigma^z_i
                q_matrix[(i_var.1, i_var.1)] = 2 * H * (if i_var.0 { 1 } else { -1 });

                // h^a_i c(i)\sigma^z_{i,a}
                let aux_variable = nb_vars + new_var_counter + i_index;
                let q_i = if i_index == 0 { G / 2 } else { 0 };
                let h_a_i = -J * (2 * i_index as QuboType - clause.len() as QuboType) + q_i;
                q_matrix[(aux_variable, aux_variable)] = 2 * h_a_i;
            }

            new_var_counter += clause.len();
        }

        for i in 0..(nb_vars + num_ancillae) {
            let coupling_sum: QuboType = (0..(nb_vars + num_ancillae))
                .filter(|j| i != *j)
                .map(|j| q_matrix[(i, j)])
                .sum();
            q_matrix[(i, i)] -= 2 * coupling_sum;
        }
        assert_eq!(num_ancillae, new_var_counter);

        debug!("Chancellor reduction produced: {}", q_matrix);

        (
            QuboProblem::try_from_q_matrix(CsrMatrix::from(&q_matrix)).unwrap(),
            Self(*nb_vars),
        )
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        SatSolution::Sat(DVector::from_fn(self.0, |i, _| solution_vector[i] != 0))
    }
}

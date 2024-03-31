use log::{trace, warn};
use nalgebra::{DMatrix, dvector, DVector};
use nalgebra_sparse::CooMatrix;
use common::Reduction;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use sat_problem::{KSatProblem, SatSolution, SatVariable};

fn fast_mod2(x: usize) -> usize {
    x & 0x1
}

pub struct Nusslein23 {
    pub nb_vars: usize,
}

impl Reduction<KSatProblem, QuboProblem> for Nusslein23 {
    fn reduce(
        &KSatProblem {
            nb_vars,
            ref clause_list,
        }: &KSatProblem,
    ) -> (QuboProblem, Self) {
        let m = clause_list.len();
        let problem_size = 2 * nb_vars + m;
        let mut q_matrix: CooMatrix<QuboType> = CooMatrix::new(problem_size, problem_size);

        let mut r_values : DVector<QuboType> = DVector::zeros(2 * nb_vars);
        let mut r_rel_values : DMatrix<QuboType> = DMatrix::zeros(2 * nb_vars, 2 * nb_vars);

        for clause in clause_list {
            for &SatVariable(is_true_i, var_i) in clause {
                let l_i = 2 * var_i + (1 - is_true_i as usize);
                r_values[l_i] += 1;
                for &SatVariable(is_true_j, var_j) in clause {
                    let l_j = 2 * var_j + (1 - is_true_j as usize);
                    if l_i != l_j {
                        r_rel_values[(l_i, l_j)] += 1;
                    }
                }
            }
        }

        r_rel_values = (r_rel_values.transpose() + r_rel_values) / 2;

        // for each var_i, i = 2 * var_i maps to var_i
        // for each var_i, i = 2 * var_i + 1 maps to ¬var_i
        for i in 0..problem_size {
            for j in i..problem_size {
                if i == j && j < 2 * nb_vars {
                    q_matrix.push(i, j, - r_values[i]);
                } else if i == j && j >= 2 * nb_vars {
                    q_matrix.push(i, j, 2);
                } else if j < 2 * nb_vars && j - i == 1 && fast_mod2(i) == 0 {
                    q_matrix.push(i, j, (m + 1) as QuboType);
                } else if i < 2 * nb_vars && j < 2 * nb_vars {
                    q_matrix.push(i, j, r_rel_values[(i, j)]);
                } else if j >= 2 * nb_vars && i < 2 * nb_vars && clause_list[j - 2 * nb_vars].iter().any(|&SatVariable(is_true, var)| 2 * var + (1 - is_true as usize) == i) {
                    q_matrix.push(i, j, -1);
                }
            }
        }

        let problem = QuboProblem::try_from_coo_matrix(&q_matrix)
            .expect("Matrix should be properly formed");

        (problem, Self {
            nb_vars
        })
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        let out_sv = DVector::from_fn(self.nb_vars, |i,_| {
            let is_true = solution_vector[2 * i] != 0;
            let is_false = solution_vector[2 * i + 1] != 0;

            if !(is_true || is_false) {
                // This variable is never chosen as the true variable in any clause, and therefore must be assumed false
                false
            } else if is_true && is_false {
                warn!("Conflict found when up-modelling for Variable {i}!");
                trace!("Model for {i} is (is_true: {is_true}) (is_false: {is_false})");
                // TODO Handle this error
                false
                // return SatSolution::Unsat;
            } else {
                is_true && !is_false
            }
        });

        SatSolution::Sat(out_sv)
    }
}

use nalgebra::DVector;
use nalgebra_sparse::{CooMatrix, CsrMatrix};
use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::problem::sat::{KSatProblem, SatSolution, SatVariable};
use crate::problem::sat::reducer::QuboToSatReduction;

pub struct Nusslein {
    constant_factor: QuboType,
    og_vars: usize
}


// fast ciel(log2(x + 1))
#[inline(always)]
fn fast_ceil_log2(x : usize) -> u32 {
    usize::BITS - x.leading_zeros()
}

fn implement_clause(problem_size: usize, mut triplets : Vec<(usize, usize, QuboType)>, mut constant_factor: QuboType, clause: &[SatVariable]) -> (usize, QuboType, Vec<(usize, usize, QuboType)>) {
    const UNIT_PENALTY: QuboType = 1;
    if clause.len() == 1 {
        let [SatVariable(is_true, var_i)] = clause[..] else {
            unreachable!()
        };
        
        triplets.push((var_i, var_i, (2 * (is_true as QuboType) - 1) * -UNIT_PENALTY));

        (problem_size, constant_factor, triplets)
    } else if clause.len() == 2 {
        // Add a two SAT Penalty (Fred Glover et al.)
        match clause[..] {
            [SatVariable(true, var_i), SatVariable(true, var_j)] => {
                // 1 - x_i - x_j + (x_i)(x_j)
                constant_factor += 1;
                triplets.push((var_i, var_i, -UNIT_PENALTY)); // -x_i
                triplets.push((var_j, var_j, -UNIT_PENALTY)); // -x_j
                // + (x_i)(x_j)
                if var_i < var_j {
                    triplets.push((var_i, var_j, UNIT_PENALTY));
                } else {
                    triplets.push((var_j, var_i, UNIT_PENALTY));
                }
            },
            [SatVariable(true, var_i), SatVariable(false, var_j)] => {
                // x_j - (x_i)(x_j)
                triplets.push((var_j, var_j, UNIT_PENALTY)); // x_j
                // - (x_i)(x_j)
                if var_i < var_j {
                    triplets.push((var_i, var_j, - UNIT_PENALTY));
                } else {
                    triplets.push((var_j, var_i, - UNIT_PENALTY));
                }
            },
            [SatVariable(false, var_i), SatVariable(true, var_j)] => {
                // x_i - (x_j)(x_i)
                triplets.push((var_i, var_i, UNIT_PENALTY)); // x_i
                // - (x_i)(x_j)
                if var_i < var_j {
                    triplets.push((var_i, var_j, - UNIT_PENALTY));
                } else {
                    triplets.push((var_j, var_i, - UNIT_PENALTY));
                }
            },
            [SatVariable(false, var_i), SatVariable(false, var_j)] => {
                // (x_i)(x_j)
                if var_i < var_j {
                    triplets.push((var_i, var_j, UNIT_PENALTY));
                } else {
                    triplets.push((var_j, var_i, UNIT_PENALTY));
                }
            }
            _ => unreachable!()
        }

        (problem_size, constant_factor, triplets)
    } else if clause.len() == 3 {
        // Add a three SAT Penalty (Chancellor et al.)
        let num_aux = clause.len();
        
        for (i, &SatVariable(is_true_i, var_i)) in clause.iter().enumerate() {
            let c_i = 2 * (is_true_i as QuboType) - 1;

            for (j, SatVariable(is_true_j, var_j)) in clause.iter().take(i).cloned().enumerate()
            {
                // First section
                let c_j = 2 * (is_true_j as QuboType) - 1;
                if var_i < var_j {
                    triplets.push((var_i, var_j, UNIT_PENALTY * c_i * c_j));
                } else {
                    triplets.push((var_j, var_i, UNIT_PENALTY * c_i * c_j));
                }

                // Auxiliary variable section one
                let j_a = problem_size + j;
                triplets.push((var_i, j_a, UNIT_PENALTY * c_i));
            }

            // Second Section
            triplets.push((var_i, var_i, - UNIT_PENALTY * c_i));

            // Auxiliary Variable section one
            let i_a = problem_size + i;

            let q_i = if i == 0 { UNIT_PENALTY / 2 } else { 0 };

            let h_a_i = - UNIT_PENALTY * ((2 * i as QuboType) - clause.len() as QuboType) + q_i;

            triplets.push((i_a, i_a, h_a_i));
        }

        (problem_size + num_aux, constant_factor, triplets)
    } else {
        let h = fast_ceil_log2(clause.len());
        
        // Create H auxiliary variables
        let mut new_clause = Vec::with_capacity(h as usize);

        // formula 6
        {
            let mut nc = 0;
            
            for &SatVariable(l, var_l) in clause {
                if !l {
                    nc += 1;
                }
                triplets.push((var_l, var_l, 2 * (l as QuboType) - 1));
            }

            for j in 0..h {
                let h_j = problem_size + j as usize;
                triplets.push((h_j, h_j, (2 as QuboType).pow(j)));

                new_clause.push(SatVariable(true, h_j))
            }
            
            constant_factor += nc;
        }

        implement_clause(problem_size + h as usize, triplets, constant_factor, &new_clause)
    }
}

impl QuboToSatReduction for Nusslein {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        let &KSatProblem {
            nb_vars,
            ref clause_list
        } = sat_problem;

        let mut problem_size = nb_vars;
        let mut triplets = Vec::new();
        let mut constant_factor = 0;
        for clause in clause_list {
            (problem_size, constant_factor, triplets) = implement_clause(problem_size, triplets, constant_factor, clause)
        }
        
        let q_matrix = {
            let (row_indices, col_indices, values) = {
                let trip_len = triplets.len();
                triplets.into_iter().fold(
                    (Vec::with_capacity(trip_len), Vec::with_capacity(trip_len), Vec::with_capacity(trip_len)),
                    |(mut i_list, mut j_list, mut v_list), (i, j, v)| {
                        i_list.push(i);
                        j_list.push(j);
                        v_list.push(v);

                        (i_list, j_list, v_list)
                    }
                )
            };
            let m = CooMatrix::try_from_triplets(problem_size, problem_size, row_indices, col_indices, values)
                .expect("Matrix should always construct correctly!");
            
            QuboProblem::try_from_q_matrix(CsrMatrix::from(&m))
                .expect("Matrix should be properly formed!")
        };
        
        (
            q_matrix,
            Self {
                og_vars: nb_vars,
                constant_factor
            }
        )
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        SatSolution::Sat(DVector::from_fn(self.og_vars, |i, _| solution_vector[i] != 0))
    }
}

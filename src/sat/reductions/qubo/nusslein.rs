use nalgebra::DVector;

use crate::core::Reduction;
use crate::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::sat::{KSatProblem, SatSolution, SatVariable};

pub struct Nusslein {
    nb_vars: usize,
}

// fast ciel(log2(x + 1))
#[inline(always)]
fn fast_ceil_log2(x: usize) -> u32 {
    usize::BITS - x.leading_zeros()
}

fn implement_clause(
    problem_size: usize,
    mut triplets: Vec<(usize, usize, QuboType)>,
    mut constant_factor: QuboType,
    clause_to_implement: &[SatVariable],
) -> (usize, QuboType, Vec<(usize, usize, QuboType)>) {
    const UNIT_PENALTY: QuboType = 1;
    match clause_to_implement[..] {
        // C MUST BE TRUE
        [SatVariable(is_true, var_i)] => {
            triplets.push((
                var_i,
                var_i,
                (2 * (is_true as QuboType) - 1) * -UNIT_PENALTY,
            ));

            (problem_size, constant_factor, triplets)
        }
        // OR(C)
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

            (problem_size, constant_factor, triplets)
        }
        [SatVariable(true, var_i), SatVariable(false, var_j)] => {
            // x_j - (x_i)(x_j)
            triplets.push((var_j, var_j, UNIT_PENALTY)); // x_j
            // - (x_i)(x_j)
            if var_i < var_j {
                triplets.push((var_i, var_j, -UNIT_PENALTY));
            } else {
                triplets.push((var_j, var_i, -UNIT_PENALTY));
            }

            (problem_size, constant_factor, triplets)
        }
        [SatVariable(false, var_i), SatVariable(true, var_j)] => {
            // x_i - (x_j)(x_i)
            triplets.push((var_i, var_i, UNIT_PENALTY)); // x_i
            // - (x_i)(x_j)
            if var_i < var_j {
                triplets.push((var_i, var_j, -UNIT_PENALTY));
            } else {
                triplets.push((var_j, var_i, -UNIT_PENALTY));
            }

            (problem_size, constant_factor, triplets)
        }
        [SatVariable(false, var_i), SatVariable(false, var_j)] => {
            // (x_i)(x_j)
            if var_i < var_j {
                triplets.push((var_i, var_j, UNIT_PENALTY));
            } else {
                triplets.push((var_j, var_i, UNIT_PENALTY));
            }

            (problem_size, constant_factor, triplets)
        }
        // 3-SAT(C)
        // 3-SAT special cases
        [SatVariable(true, var_i), SatVariable(true, var_j), SatVariable(true, var_k)] => {
            if var_i < var_j {
                triplets.push((var_i, var_j, 2));
            } else {
                triplets.push((var_j, var_i, 2));
            }
            triplets.push((var_i, problem_size, -2));
            triplets.push((var_j, problem_size, -2));
            triplets.push((var_k, var_k, -1));
            triplets.push((var_k, problem_size, 1));
            triplets.push((problem_size, problem_size, 1));

            (problem_size + 1, constant_factor, triplets)
        }
        [SatVariable(true, var_i), SatVariable(true, var_j), SatVariable(false, var_k)]
        | [SatVariable(true, var_i), SatVariable(false, var_k), SatVariable(true, var_j)]
        | [SatVariable(false, var_k), SatVariable(true, var_i), SatVariable(true, var_j)] => {
            if var_i < var_j {
                triplets.push((var_i, var_j, 2));
            } else {
                triplets.push((var_j, var_i, 2));
            }
            triplets.push((var_i, problem_size, -2));
            triplets.push((var_j, problem_size, -2));
            triplets.push((var_k, var_k, 1));
            triplets.push((var_k, problem_size, -1));
            triplets.push((problem_size, problem_size, 2));

            (problem_size + 1, constant_factor, triplets)
        }
        [SatVariable(true, var_i), SatVariable(false, var_j), SatVariable(false, var_k)]
        | [SatVariable(false, var_j), SatVariable(false, var_k), SatVariable(true, var_i)]
        | [SatVariable(false, var_j), SatVariable(true, var_i), SatVariable(false, var_k)] => {
            triplets.push((var_i, var_i, 2));
            if var_i < var_j {
                triplets.push((var_i, var_j, -2));
            } else {
                triplets.push((var_j, var_i, -2));
            }
            triplets.push((var_i, problem_size, -2));
            triplets.push((var_j, problem_size, 2));
            triplets.push((var_k, var_k, 1));
            triplets.push((var_k, problem_size, -1));

            (problem_size + 1, constant_factor, triplets)
        }
        [SatVariable(false, var_j), SatVariable(false, var_i), SatVariable(false, var_k)] => {
            triplets.push((var_i, var_i, -1));
            if var_i < var_j {
                triplets.push((var_i, var_j, 1));
            } else {
                triplets.push((var_j, var_i, 1));
            }
            if var_i < var_k {
                triplets.push((var_i, var_k, 1));
            } else {
                triplets.push((var_k, var_i, 1));
            }
            triplets.push((var_i, problem_size, 1));
            triplets.push((var_j, var_j, -1));
            if var_j < var_k {
                triplets.push((var_j, var_k, 1));
            } else {
                triplets.push((var_k, var_j, 1));
            }
            triplets.push((var_j, problem_size, 1));
            triplets.push((var_k, var_k, -1));
            triplets.push((var_k, problem_size, 1));
            triplets.push((problem_size, problem_size, 1));

            (problem_size + 1, constant_factor, triplets)
        }
        // Formula 6
        ref clause => {
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

            implement_clause(
                problem_size + h as usize,
                triplets,
                constant_factor,
                &new_clause,
            )
        }
    }
}

impl Reduction<KSatProblem, QuboProblem> for Nusslein {
    fn reduce(
        &KSatProblem {
            nb_vars,
            ref clause_list,
        }: &KSatProblem,
    ) -> (QuboProblem, Self) {
        let mut problem_size = nb_vars;
        let mut triplets = Vec::new();
        for clause in clause_list {
            (problem_size, _, triplets) =
                implement_clause(problem_size, triplets, 0, clause)
        }

        let q_matrix = QuboProblem::try_from_triplets(problem_size, triplets)
            .expect("Matrix should be properly constructed.");

        (
            q_matrix,
            Self {
                nb_vars
            },
        )
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        SatSolution::Sat(DVector::from_fn(self.nb_vars, |i, _| {
            solution_vector[i] != 0
        }))
    }
}

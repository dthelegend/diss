use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution, SatVariable};
use nalgebra::DVector;

pub struct Chancellor(usize);

pub fn implement_clause(
    problem_size: usize,
    mut triplets: Vec<(usize, usize, QuboType)>,
    mut biases: Vec<(usize, QuboType)>,
    clause: &[SatVariable],
) -> (usize, Vec<(usize, usize, QuboType)>, Vec<(usize, QuboType)>) {
    if clause.len() == 3 {
        const J: QuboType = 5;
        const J_A: QuboType = 2 * J; // J_A = 2J > |H|
        const G: QuboType = 1;
        const H: QuboType = G;
        const H_A: QuboType = 2 * H;

        let var_a = problem_size;
        let mut c_a = -1;

        for (i, &SatVariable(is_true_i, var_i)) in clause.iter().enumerate() {
            let c_i = 2 * (is_true_i as QuboType) - 1;

            c_a *= c_i;

            for &SatVariable(is_true_j, var_j) in &clause[(i + 1)..] {
                let c_j = 2 * (is_true_j as QuboType) - 1;

                // This line is correct
                triplets.push((var_i, var_j, J + H * c_i * c_j));
            }

            // This line is correct
            triplets.push((var_i, var_a, J_A));
        }

        // + h^a sigma_a^z
        // This is correct
        biases.push((var_a, H_A * c_a));

        {
            // TODO replace hard coded with a real calculation
            match clause[..] {
                [SatVariable(true, var_i), SatVariable(true, var_j), SatVariable(true, var_k)] => {
                    biases.push((var_i, -2));
                    biases.push((var_j, -2));
                    biases.push((var_k, -2));
                }
                [SatVariable(false, var_i), SatVariable(false, var_j), SatVariable(false, var_k)] =>
                {
                    biases.push((var_i, 2));
                    biases.push((var_j, 2));
                    biases.push((var_k, 2));
                }
                [SatVariable(true, var_i), SatVariable(true, var_j), SatVariable(true, var_k)] => {
                    biases.push((var_i, 2));
                    biases.push((var_j, 2));
                    biases.push((var_k, 2));
                }
                [SatVariable(true, var_i), SatVariable(true, var_j), SatVariable(false, var_k)] => {
                    biases.push((var_i, 0));
                    biases.push((var_j, 0));
                    biases.push((var_k, 2));
                }
                [SatVariable(true, var_i), SatVariable(false, var_j), SatVariable(true, var_k)] => {
                    biases.push((var_i, 0));
                    biases.push((var_j, 2));
                    biases.push((var_k, 0));
                }
                [SatVariable(false, var_i), SatVariable(true, var_j), SatVariable(true, var_k)] => {
                    biases.push((var_i, 2));
                    biases.push((var_j, 0));
                    biases.push((var_k, 0));
                }
                [SatVariable(true, var_i), SatVariable(false, var_j), SatVariable(false, var_k)] => {
                    biases.push((var_i, -2));
                    biases.push((var_j, 0));
                    biases.push((var_k, 0));
                }
                [SatVariable(false, var_i), SatVariable(true, var_j), SatVariable(false, var_k)] => {
                    biases.push((var_i, 0));
                    biases.push((var_j, -2));
                    biases.push((var_k, 0));
                }
                [SatVariable(false, var_i), SatVariable(false, var_j), SatVariable(true, var_k)] => {
                    biases.push((var_i, 0));
                    biases.push((var_j, 0));
                    biases.push((var_k, -2));
                }
                _ => unreachable!(),
            }
        }

        (problem_size + 1, triplets, biases)
    } else {
        unimplemented!()
    }
}

impl QuboToSatReduction for Chancellor {
    fn reduce(
        &KSatProblem {
            nb_vars,
            ref clause_list,
        }: &KSatProblem,
    ) -> (QuboProblem, Self) {
        let mut problem_size = nb_vars;
        let mut j_triplets = Vec::new();
        let mut j_biases = Vec::new();

        for clause in clause_list {
            (problem_size, j_triplets, j_biases) =
                implement_clause(problem_size, j_triplets, j_biases, clause);
        }

        let (q_matrix, _) =
            QuboProblem::try_from_ising_triplets(problem_size, j_triplets, j_biases)
                .expect("Matrix should be properly constructed.");
        // let (q_matrix, _) = QuboProblem::try_from_ising_triplets(4, vec![(0,1,2), (0,3,-2), (1,3,-2), (2,3,1)], vec![(2, -1),(3, -1)]).unwrap();

        (q_matrix, Self(nb_vars))
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        SatSolution::Sat(DVector::from_fn(self.0, |i, _| solution_vector[i] != 0))
    }
}

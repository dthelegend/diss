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
        const H_A : QuboType = 2 * H;

        let var_a = problem_size;

        let mut c_a = -1;
        
        for (i, &SatVariable(is_true_i, var_i)) in clause.iter().enumerate() {
            let c_i = 2 * (is_true_i as QuboType) - 1;

            // single terms (a_i)
            // TODO THIS IS WRONG
            biases.push((var_i, - J_A * c_i));

            // double terms - (a_i)(a_j) =
            // THIS IS RIGHT
            for &SatVariable(is_true_j, var_j) in &clause[(i + 1)..] {
                let c_j = 2 * (is_true_j as QuboType) - 1;

                triplets.push((var_i, var_j, J * c_i * c_j + H));
            }
            
            // triple term
            // TODO THIS IS WRONG
            biases.push((var_i, H * c_i));
            triplets.push((var_i, var_a, J_A));
            
            c_a *= c_i
        }
        // + h^a sigma_a^z
        biases.push((var_a, H_A * c_a));

        // println!("{triplets:?}");

        (problem_size + 1, triplets, biases)
    } else {
        todo!()
        // // For clause var
        // for (i, &SatVariable(is_true_i, var_i)) in clause.iter().enumerate() {
        //     let c_i = 2 * (is_true_i as QuboType) - 1;
        //
        //     // Initialise this clause
        //     triplets.push((var_i, var_i, H * 2 * c_i));
        //     for (j, &SatVariable(is_true_j, var_j)) in clause.iter().enumerate() {
        //         // Bind it to the other clauses
        //         let c_j = 2 * (is_true_j as QuboType) - 1;
        //         if j < i {
        //             if var_i < var_j {
        //                 triplets.push((var_i, var_j, 4 * J * c_i * c_j));
        //             } else {
        //                 triplets.push((var_j, var_i, 4 * J * c_i * c_j));
        //             }
        //             triplets.push((var_j, var_j, -2 * J * c_i * c_j));
        //             triplets.push((var_i, var_i, -2 * J * c_i * c_j));
        //         }
        //
        //         // Bind to auxiliary variable
        //         triplets.push((var_j, problem_size, 4 * J_A * c_i));
        //         triplets.push((var_j, problem_size, -2 * J_A * c_i));
        //         triplets.push((var_i, problem_size, -2 * J_A * c_i));
        //     }
        //
        //     triplets.push((var_i, problem_size, 2 * c_i * J_A));
        // }
        //
        // // Create auxiliary variable
        // triplets.push((problem_size, problem_size, H_A));
    }
}

impl QuboToSatReduction for Chancellor {
    fn reduce(&KSatProblem { nb_vars, ref clause_list }: &KSatProblem) -> (QuboProblem, Self) {
        let mut problem_size = nb_vars;
        let mut j_triplets = Vec::new();
        let mut j_biases = Vec::new();

        for clause in clause_list {
            (problem_size, j_triplets, j_biases) = implement_clause(problem_size, j_triplets, j_biases, clause);
        }

        let (q_matrix, _) = QuboProblem::try_from_hamiltonian_triplets(problem_size, j_triplets, j_biases)
            .expect("Matrix should be properly constructed.");

        // assert_eq!(num_ancillae, new_var_counter);
        (
            q_matrix,
            Self(nb_vars),
        )
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        SatSolution::Sat(DVector::from_fn(self.0, |i, _| solution_vector[i] != 0))
    }
}

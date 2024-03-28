use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use crate::problem::sat::reducer::QuboToSatReduction;
use crate::problem::sat::{KSatProblem, SatSolution, SatVariable};
use nalgebra::DVector;

pub struct Chancellor(usize);

type ClauseTripletBias = (usize, Vec<(usize, usize, QuboType)>, Vec<(usize, QuboType)>);

pub fn implement_clause(
    problem_size: usize,
    mut triplets: Vec<(usize, usize, QuboType)>,
    mut biases: Vec<(usize, QuboType)>,
    clause: &[SatVariable],
) -> ClauseTripletBias {
    match clause[..] {
        ref c @
        [SatVariable(is_true_i, var_i), SatVariable(is_true_j, var_j), SatVariable(is_true_k, var_k)] =>
        {
            const J: QuboType = 5;
            const J_A: QuboType = 2 * J; // J_A = 2J > |H|
            const G: QuboType = 1;
            const H: QuboType = G;
            const H_A: QuboType = 2 * H;

            let mut test_biases = Vec::new();

            let var_a = problem_size;
            let mut c_a = -1;

            // TODO REMOVE C
            for (i, &SatVariable(is_true_i, var_i)) in c.iter().enumerate() {
                let c_i = 2 * (is_true_i as QuboType) - 1;

                c_a *= c_i;

                test_biases.push((var_i, H * c_i));
                for &SatVariable(is_true_j, var_j) in &c[(i + 1)..] {
                    let c_j = 2 * H * (is_true_j as QuboType) - 1;

                    // This line is correct
                    triplets.push((var_i, var_j, J + H * c_i * c_j));
                }

                // This line is correct
                triplets.push((var_i, var_a, J_A));
            }

            // + h^a sigma_a^z
            // This is correct
            biases.push((var_a, H_A * c_a));

            let c_i = 2 * (is_true_i as QuboType) - 1;
            let c_j = 2 * (is_true_j as QuboType) - 1;
            let c_k = 2 * (is_true_k as QuboType) - 1;

            biases.push((var_i, -2 * H * c_i * !(is_true_j ^ is_true_k) as QuboType));
            biases.push((var_j, -2 * H * c_j * !(is_true_i ^ is_true_k) as QuboType));
            biases.push((var_k, -2 * H * c_k * !(is_true_i ^ is_true_j) as QuboType));

            (problem_size + 1, triplets, biases)
        }
        _ => unimplemented!(),
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

use log::debug;
use nalgebra::{DMatrix, DVector};
use rand::distributions::Bernoulli;
use common::Solver;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use rand::prelude::*;
use rand_distr::Gamma;
use rayon::prelude::*;

type MaType = f32;

mod mopso_gpu {
    use qubo_problem::{QuboProblem, QuboType};
    use rand::Rng;

    #[link(name = "kernels")]
    extern "C" {
        fn run_mopso_solver(
            problem_size: usize,
            qubo_problem: *const QuboType,
            solutions_flat: *const QuboType,
            number_of_particles: usize,
        );
    }

    pub fn gpu_mopso_helper(
        mut rng: impl Rng,
        qubo_problem: &QuboProblem,
        number_of_particles: usize,
    ) {
        // let solutions_flat: DMatrix<QuboType> =
        //     DMatrix::from_fn(number_of_particles, qubo_problem.get_size(), |_, _| {
        //         rng.gen_range(0..=1)
        //     });

        // unsafe {
        //     run_mopso_solver(
        //         qubo_problem.get_size(),
        //         dense_problem.as_ptr(),
        //         solutions_flat.as_ptr(),
        //         number_of_particles,
        //     )
        // }

        todo!()
    }
}

fn power_iteration(A: &DMatrix<MaType>, epsilon: MaType, max_iterations: usize) -> MaType {
    let mut b = DVector::new_random(A.nrows());

    for _ in 0..max_iterations {
        b = A * b;
    }
    
    // b
    todo!()
}

pub struct MomentumAnnealer
{
    max_iterations: usize,
}

impl MomentumAnnealer
{
    pub fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations,
        }
    }
}

fn dropout(k: usize) -> MaType {
    MaType::max(0.0, 0.5 - (k as MaType / 2000.0))
}

fn momentum_scaling_factor(k : usize) -> MaType {
    MaType::min(1.0, MaType::sqrt(k as MaType / 1000.0))
}

fn temperature(k: usize) -> MaType {
    const BETA_0 : MaType = 0.0003;
    
    1.0 / (BETA_0 * MaType::ln(1.0 + k as MaType))
}

impl Solver<QuboProblem> for MomentumAnnealer
{
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        let (h_bias, j_mat) = {
            let (q_typed_bias, q_typed_mat) = qubo_problem.get_ising();

            (q_typed_bias.cast(), q_typed_mat.cast())
        };

        let max_eigenvalue: MaType =
            // According to the paper this should not take longer than 300 iterations to be close
            // enough to optimal
            power_iteration(&(- &j_mat), 1e-6 , 1000);


        debug!("Starting to momentum anneal");

        let problem_size = qubo_problem.get_size();

        let w = {
            let mut w_builder = DVector::zeros(problem_size);
            let mut c : DVector<MaType> = DVector::zeros(problem_size);

            for i in 0..problem_size {
                let abs_row_sum = j_mat.row(i).map(MaType::abs).sum();
                if max_eigenvalue >= abs_row_sum {
                    w_builder[i] = abs_row_sum;
                    c[i] = 1.0;
                } else {
                    w_builder[i] = max_eigenvalue / 2.0;
                }
            }

            let neg_vec = DVector::from_fn(problem_size, |i,_| (j_mat.row(i).map(MaType::abs) * &c)[0] / 2.0);
            
            println!("{max_eigenvalue}\n{neg_vec}\n{c}");
            
            w_builder -= neg_vec;

            w_builder
        };

        let gamma = Gamma::new(1.0,1.0)
            .unwrap();

        let mut s_k : DVector<f32> = DVector::from_distribution(problem_size, &Bernoulli::new(0.5).unwrap(), &mut thread_rng()).map(|x| if x { 1.0 } else { -1.0 });
        let mut s_k1 : DVector<f32> = s_k.clone();
        
        for k in 1..=self.max_iterations {
            let c_k = momentum_scaling_factor(k);
            let p_k = dropout(k);
            let t_k = temperature(k);

            let bernoulli = Bernoulli::new(p_k as f64).unwrap();

            let c_k_vector = DVector::repeat(problem_size, c_k);

            let dropout_vector = DVector::from_distribution(problem_size, &bernoulli, &mut thread_rng()).map(|x| if x { 1.0 } else { 0.0 });

            let temp_w = w.component_mul(&dropout_vector.component_mul(&c_k_vector));

            let gamma_k : DVector<f32> = DVector::from_distribution(problem_size, &gamma, &mut thread_rng());

            let j_mat_plus_w_diag = {
                let mut j_mat_plus_w_diag_builder = j_mat.clone();
                j_mat_plus_w_diag_builder.set_diagonal(&temp_w);

                j_mat_plus_w_diag_builder
            };
            
            s_k = (&h_bias
                + j_mat_plus_w_diag * &s_k1
                - gamma_k.component_mul(&s_k).scale(t_k / 2.0)).map(MaType::signum);
            
            (s_k, s_k1) = (s_k1, s_k);
        }

        // gpu_mopso_helper(&mut self.rng, qubo_problem, self.number_of_particles);

        QuboSolution(s_k.map(|x| (x as QuboType + 1) / 2))
    }
}

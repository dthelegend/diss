use std::iter::zip;
use log::debug;
use nalgebra::{DMatrix, DVector};
use common::Solver;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use rand::prelude::*;
use rand_distr::Gamma;
use rayon::prelude::*;

mod mopso_gpu {
    use nalgebra::DMatrix;
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
        let solutions_flat: DMatrix<QuboType> =
            DMatrix::from_fn(number_of_particles, qubo_problem.get_size(), |_, _| {
                rng.gen_range(0..=1)
            });

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

fn power_iteration(matrix: &DMatrix<f64>, epsilon: f64, max_iterations: usize) -> f64 {
    let mut x = DVector::repeat(matrix.nrows(), 1.0);
    let mut lambda_old = 0.0;

    for _ in 0..max_iterations {
        let y = matrix * x;
        let lambda = y.norm();

        x = y / lambda;

        if (lambda - lambda_old).abs() < epsilon {
            break;
        }

        lambda_old = lambda;
    }

    lambda_old
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


fn dropout(k: usize) -> f64 {
    f64::max(0f64, 0.5 - (k as f64) / 2000f64)
}

fn momentum_scaling_factor(k : usize) -> f64 {
    f64::min(1.0, f64::sqrt(k as f64 / 1000f64))
}

fn temperature(k: usize) -> f32 {
    const BETA_0 : f32 = 0.1;
    
    1.0 / (BETA_0 * f32::ln(1 + k))
}

fn fast_is_odd_as_usize(k: usize) -> usize {
    k & 0x1
}

impl Solver<QuboProblem> for MomentumAnnealer
{
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        let max_eigenvalue: f32 =
            power_iteration(&(-qubo_problem.get_dense().cast()), 1e-6 , 1000);


        let (h_bias, j_mat) = {
            let mut h_bias_builder: DVector<QuboType> = DVector::zeros(qubo_problem.get_size());
            let mut j_mat_builder: DMatrix<QuboType> = DMatrix::zeros(qubo_problem.get_size(), qubo_problem.get_size());

            for (i, j, &v) in qubo_problem.get_sparse().upper_triangle().triplet_iter() {
                if i == j {
                    h_bias_builder[i] += v;
                } else {
                    j_mat_builder[(i, j)] += v;

                    h_bias_builder[i] += v;
                    h_bias_builder[j] += v;
                }
            }

            (h_bias_builder, j_mat_builder)
        };

        debug!("Starting to momentum anneal");

        let problem_size = qubo_problem.get_size();

        let w = {
            let mut w_builder = DVector::zeros(problem_size);

            let mut c = vec![false; problem_size];

            for i in 0..problem_size {
                let row_sum = j_mat.row(i).sum();
                if max_eigenvalue >= row_sum {
                    w_builder[i] = row_sum;
                    c[i] |= true;
                } else {
                    w_builder[i] = max_eigenvalue / 2.0 ;
                }
            }

            for i in c.iter().cloned().enumerate().filter_map(|(i, v)| if v { Some(i) } else { None }) {
                let row_i = j_mat.row(i);

                w_builder[i] -= zip(row_i.iter().cloned(), c.iter().cloned()).map(|(v_i, c_i)| v_i * (c_i as u8) as f32 / 2).sum::<f32>();
            }

            w_builder.transpose()
        };

        let gamma = Gamma::new(1.0,1.0)
            .unwrap();

        let mut s_k : DVector<f32> = DVector::zeros(problem_size * 2);
        for i in 0..problem_size {
            let v = (thread_rng().gen_range(0..=1) * 2 - 1) as f32;
            s_k[2 * i] = v;
            s_k[2 * i + 1] = v;
        };

        let hb_f32 = h_bias.clone().cast::<f32>();

        for k in 0..=self.max_iterations {
            let c_k = momentum_scaling_factor(k);
            let p_k = dropout(k);
            let t_k = temperature(k);

            let temp_w = DVector::from_vec(w.par_column_iter()
               .map(|x| ((x[0] * (thread_rng().gen_bool(1.0 - p_k))) * c_k).ceil() as QuboType)
               .collect());

            let gamma_k : DVector<f32> = DVector::from_vec((0..problem_size).into_par_iter().map(|_| gamma.sample(&mut thread_rng())).collect());

            let j_mat_plus_w_diag = {
                let mut j_mat_plus_w_diag_builder = j_mat.cast::<f32>();
                j_mat_plus_w_diag_builder.set_diagonal(&temp_w);

                j_mat_plus_w_diag_builder
            };

            let side = fast_is_odd_as_usize(k);
            let other_side = 1 - side;

            let new_sk = &hb_f32
                + j_mat_plus_w_diag * s_k.rows_with_step(other_side, problem_size, 1)
                - gamma_k.scale(t_k / 2.0).component_mul(&(s_k.rows_with_step(side, problem_size, 1)));

            s_k.rows_with_step_mut(side, problem_size, 1)
                .set_column(0, &new_sk.map(|x| x.signum() as QuboType));
        }

        // gpu_mopso_helper(&mut self.rng, qubo_problem, self.number_of_particles);

        QuboSolution(s_k.rows_with_step(fast_is_even_as_usize(self.max_iterations), problem_size, 1).map(|x| (x + 1) / 2))
    }
}

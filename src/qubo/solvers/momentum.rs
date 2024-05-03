use log::{debug, trace};
use nalgebra::{DMatrix, DVector};
use rand::distributions::Bernoulli;
use rand::prelude::*;
use rand_distr::Gamma;
use rayon::prelude::*;

use crate::core::Solver;
use crate::qubo::{QuboProblem, QuboSolution, QuboType};

type MaType = f32;

fn power_method(matrix: &DMatrix<MaType>, epsilon: MaType, max_iter: usize) -> MaType {
    let n = matrix.ncols();
    let mut x = DVector::repeat(n, 1.0);

    let mut lambda: MaType = 1.0;
    let mut lambda_prev: MaType;

    for _ in 0..max_iter {
        x = (matrix * x).normalize();
        lambda_prev = lambda;
        lambda = (x.transpose() * matrix * &x)[0];

        if (lambda - lambda_prev).abs() < epsilon {
            break;
        }
    }

    lambda
}

fn largest_eigenvalue(j_mat: &DMatrix<MaType>, epsilon: MaType, max_iterations: usize) -> MaType {
    let mut mu = j_mat.abs().row_sum().max() / 100.0;

    let mut k_largest_eigenvalue = 0.0;
    while k_largest_eigenvalue <= 0.0 {
        mu += -k_largest_eigenvalue;
        let k_mat = j_mat + DMatrix::identity(j_mat.nrows(), j_mat.ncols()).scale(mu);

        k_largest_eigenvalue = power_method(&k_mat, epsilon, max_iterations);
    }

    k_largest_eigenvalue - mu
}

pub struct MomentumAnnealer
{
    max_iterations: usize,
}

impl MomentumAnnealer
{
    pub fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations
        }
    }
}

fn dropout(k: usize) -> MaType {
    MaType::max(0.0, 0.5 - (k as MaType / 2000.0))
}

fn momentum_scaling_factor(k: usize) -> MaType {
    MaType::min(1.0, MaType::sqrt(k as MaType / 1000.0))
}

fn temperature(k: usize) -> MaType {
    const BETA_0: MaType = 1e-6;

    1.0 / (BETA_0 * MaType::ln(1.0 + k as MaType))
}

impl Solver<QuboProblem> for MomentumAnnealer
{
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        let (h_bias, j_mat): (DVector<MaType>, DMatrix<MaType>) = {
            let (q_typed_bias, q_typed_mat, _q_offset) = qubo_problem.get_ising();

            (q_typed_bias.cast(), q_typed_mat.cast())
        };

        trace!("Generated J-Matrix and bias {j_mat}{}", h_bias.transpose());

        let j_mat_sym = j_mat.transpose() + j_mat;

        let max_eigenvalue: MaType =
            // According to the paper this should not take longer than 300 iterations to be close
            // enough to the real value and is thus a constant factor
            largest_eigenvalue(&(-&j_mat_sym), 1e-6, 300);

        debug!("Using maximum eigenvalue {max_eigenvalue}");

        let problem_size = qubo_problem.get_size();

        let w = {
            let mut w_builder = DVector::zeros(problem_size);
            let mut c: DVector<MaType> = DVector::zeros(problem_size);

            for i in 0..problem_size {
                let abs_row_sum = j_mat_sym.row(i).map(MaType::abs).sum();
                if max_eigenvalue >= abs_row_sum {
                    w_builder[i] = abs_row_sum;
                    c[i] = 1.0;
                } else {
                    w_builder[i] = max_eigenvalue / 2.0;
                }
            }

            let neg_vec = DVector::from_fn(problem_size, |i, _| c[i] * (j_mat_sym.row(i).map(MaType::abs) * &c)[0] / 2.0);

            w_builder -= neg_vec;

            w_builder
        };

        let gamma = Gamma::new(1.0, 1.0)
            .unwrap();

        let mut s_k: DVector<MaType> = DVector::from_distribution(problem_size, &Bernoulli::new(0.5).unwrap(), &mut thread_rng()).map(|x| if x { 1.0 } else { -1.0 });
        let mut s_k1: DVector<MaType> = s_k.clone();

        for k in 0..=self.max_iterations {
            let c_k = momentum_scaling_factor(k);
            let p_k = dropout(k);
            let t_k = temperature(k);

            let bernoulli = Bernoulli::new(p_k as f64).unwrap();

            s_k.par_column_iter_mut().enumerate().for_each(|(i, mut s_ki)| {
                let mut rng = thread_rng();
                let gamma_k = gamma.sample(&mut rng);
                let should_drop = bernoulli.sample(&mut rng);

                let jsk: MaType = (j_mat_sym.row(i) * &s_k1)[0] + if should_drop { 0.0 } else { w[i] * s_k1[i] * c_k };

                s_ki[0] = (h_bias[i] + jsk - (t_k / 2.0) * gamma_k * s_ki[0]).signum()
            });

            (s_k, s_k1) = (s_k1, s_k);
        }

        let final_solution = s_k.map(|x| (x as QuboType + 1) / 2);

        debug!("Final Evaluation is {}", final_solution.transpose());

        QuboSolution(final_solution)
    }
}

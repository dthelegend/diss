use std::cmp::{max_by_key, min_by_key};
use std::num::NonZeroUsize;

use nalgebra::DVector;
use rand::prelude::IteratorRandom;
use rand::{Rng, thread_rng};
use rayon::prelude::*;

use crate::core::Solver;
use crate::logging::log_solver_performance;
use crate::qubo::{QuboProblem, QuboSolution, QuboType};

pub struct SimulatedAnnealer
{
    max_iterations: NonZeroUsize,
    parallelism: NonZeroUsize
}

impl SimulatedAnnealer
{
    pub fn new(max_iterations: NonZeroUsize, parallelism: NonZeroUsize) -> Self {
        Self {
            max_iterations,
            parallelism
        }
    }
}

fn temperature(x: f64) -> f64 {
    const K: f64 = 5.0;

    f64::exp(-x * K)
}

impl Solver<QuboProblem> for SimulatedAnnealer {
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        (0..self.parallelism.get()).into_par_iter().map(|_| {
            let mut rng = thread_rng();
            let mut current_solution =
                QuboSolution(DVector::from_fn(qubo_problem.get_size(), |_, _| {
                    rng.gen_range(0..=1)
                }));
            let mut current_evaluation = qubo_problem.evaluate(&current_solution);
            let mut current_deltas: Vec<_> = (0..qubo_problem.get_size())
                .map(|i| qubo_problem.delta_evaluate_k(&current_solution, i))
                .collect();

            let mut best_solution = current_solution.clone();
            let mut best_evaluation = current_evaluation;


            let max_k = self.max_iterations.get();
            for k in 0..max_k {
                log_solver_performance(current_evaluation, k);
                
                let t = temperature((k + 1) as f64 / (max_k as f64));

                let ((min_delta_i, min_delta), (_, max_delta)) = current_deltas
                    .iter()
                    .cloned()
                    .enumerate()
                    .fold(None, |m, x| {
                        m.map_or(Some((x, x)), |(m1, m2)| {
                            Some((
                                min_by_key(m1, x, |(_, v)| *v),
                                max_by_key(m2, x, |(_, v)| *v),
                            ))
                        })
                    })
                    .expect("Deltas cannot be empty!");
                let p = rng.gen_range(0f64..t);
                let max_acceptable_value =
                    ((1.0 - p) * min_delta as f64 + p * max_delta as f64).ceil() as QuboType;

                let valid_choices = current_deltas
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|(_, x)| *x <= max_acceptable_value)
                    .map(|(i, _)| i);

                let random_i = valid_choices
                    .choose(&mut rng)
                    .expect("Choices cannot be empty (It must at least have min)");

                let random_evaluation = current_evaluation + current_deltas[random_i];

                let min_evaluation = current_evaluation + min_delta;
                if min_evaluation < best_evaluation {
                    best_solution = current_solution.flip(min_delta_i);
                    best_evaluation = min_evaluation;
                }

                // Calculating deltas must be done before updating the solution!
                current_deltas = current_deltas
                    .into_iter()
                    .enumerate()
                    .map(|(j, d_j)| {
                        qubo_problem.flip_j_and_delta_evaluate_k(&current_solution, d_j, random_i, j)
                    })
                    .collect();

                current_evaluation = random_evaluation;
                current_solution = current_solution.flip(random_i);
                
            }
            
            log_solver_performance(current_evaluation, max_k);

            (best_solution, best_evaluation)
        })
            .min_by_key(|(_,x)| *x)
            .expect("Parallelism cannot be zero!")
            .0
    }
}

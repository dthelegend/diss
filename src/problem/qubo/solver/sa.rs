use log::{debug, trace};
use nalgebra::DVector;
use rand::prelude::IteratorRandom;
use std::cmp::{max_by_key, min_by_key};

use crate::problem::qubo::solver::QuboSolver;
use crate::problem::qubo::{QuboProblem, QuboSolution, QuboType};
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct SimulatedAnnealer<Rng>
where
    Rng: rand::Rng,
{
    rng: Rng,
    max_iterations: usize,
}

impl<Rng> SimulatedAnnealer<Rng>
where
    Rng: rand::Rng,
{
    pub fn new_with_rng(rng: Rng, max_iterations: usize) -> Self {
        Self {
            rng,
            max_iterations,
        }
    }
}

impl SimulatedAnnealer<ThreadRng> {
    pub fn new_with_thread_rng(max_iterations: usize) -> Self {
        Self::new_with_rng(thread_rng(), max_iterations)
    }
}

fn temperature(x: f64) -> f64 {
    const K: f64 = 5.0;

    f64::exp(-x * K)
}

impl<Rng> QuboSolver for SimulatedAnnealer<Rng>
where
    Rng: rand::Rng,
{
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        let mut current_solution =
            QuboSolution(DVector::from_fn(qubo_problem.get_size(), |_, _| {
                self.rng.gen_range(0..=1)
            }));
        let mut current_evaluation = qubo_problem.evaluate(&current_solution);
        let mut current_deltas: Vec<_> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&current_solution, i))
            .collect();

        let mut best_solution = current_solution.clone();
        let mut best_evaluation = current_evaluation;

        for k in 1..=self.max_iterations {
            let t = temperature(k as f64 / (self.max_iterations as f64));

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
            let p = self.rng.gen_range(0f64..t);
            let max_acceptable_value =
                ((1.0 - p) * min_delta as f64 + p * max_delta as f64).ceil() as QuboType;

            let valid_choices = current_deltas
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, x)| *x <= max_acceptable_value)
                .map(|(i, _)| i);

            let random_i = valid_choices
                .choose(&mut self.rng)
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

            trace!("Current evaluation is {current_evaluation}");
        }

        debug!("Final Evaluation is {best_evaluation}");

        best_solution
    }
}

use log::{debug, trace};
use nalgebra::DVector;

use crate::problem::qubo::solver::QuboSolver;
use crate::problem::qubo::{QuboProblem, QuboSolution};
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

fn acceptance_probability(evaluation: isize, other_evaluation: isize, temperature: f64) -> f64 {
    f64::exp(-(other_evaluation - evaluation) as f64 / temperature)
}

impl<Rng> QuboSolver for SimulatedAnnealer<Rng>
where
    Rng: rand::Rng,
{
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        let mut current_solution =
            QuboSolution(DVector::from_fn(qubo_problem.get_size(), |_, _| {
                self.rng.gen_range(0..1)
            }));
        let mut current_evaluation = qubo_problem.evaluate(&current_solution);
        let mut current_deltas: Vec<_> = (0..qubo_problem.get_size())
            .map(|i| qubo_problem.delta_evaluate_k(&current_solution, i))
            .collect();

        let mut best_solution = current_solution.clone();
        let mut best_evaluation = current_evaluation;

        for k in 0..self.max_iterations {
            let t = temperature((k + 1) as f64 / (self.max_iterations as f64));
            
            let random_i = self.rng.gen_range(0..qubo_problem.get_size());

            let random_evaluation = current_evaluation + current_deltas[random_i];

            if random_evaluation < best_evaluation {
                best_solution = current_solution.flip(random_i);
                best_evaluation = random_evaluation;
            }
            
            if acceptance_probability(current_evaluation, random_evaluation, t)
                > self.rng.gen_range(0f64..1f64)
            {
                // Calculating deltas must be done before updating the solution!
                current_deltas = current_deltas
                    .into_iter()
                    .enumerate()
                    .map(|(j, d_j)| {
                        qubo_problem.flip_j_and_delta_evaluate_k(
                            &current_solution,
                            d_j,
                            random_i,
                            j,
                        )
                    })
                    .collect();

                current_evaluation = random_evaluation;
                current_solution = current_solution.flip(random_i);
            }

            trace!("Current evaluation is {current_evaluation}");
        }

        debug!("Final Evaluation is {best_evaluation}");

        best_solution
    }
}

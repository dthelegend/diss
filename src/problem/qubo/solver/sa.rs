use log::{debug, trace};
use nalgebra::{DVector};

use rand::rngs::ThreadRng;
use rand::thread_rng;
use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::qubo::solver::QuboSolver;

pub struct SimulatedAnnealer<Rng> where Rng : rand::Rng {
    rng: Rng,
    max_iterations: usize
}

impl <Rng> SimulatedAnnealer<Rng> where Rng : rand::Rng {
    pub fn new_with_rng(rng: Rng, max_iterations: usize) -> Self {
        Self {
            rng,
            max_iterations
        }
    }
}

impl SimulatedAnnealer<ThreadRng> {
    pub fn new_with_thread_rng(max_iterations: usize) -> Self {
        Self::new_with_rng(thread_rng(), max_iterations)
    }
}

fn temperature(x: f64) -> f64 {
    x
}

fn acceptance_probability(evaluation: isize, other_evaluation: isize, temperature: f64) -> f64 {
    f64::exp(-(other_evaluation - evaluation) as f64 / temperature)
}

impl <Rng> QuboSolver for SimulatedAnnealer<Rng> where Rng : rand::Rng {

    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        let mut current_solution = QuboSolution(DVector::from_fn(qubo_problem.get_size(), |_, _| self.rng.gen_range(0..1)));
        let mut current_evaluation = qubo_problem.evaluate(&current_solution);

        let mut best_solution = current_solution.clone();
        let mut best_evaluation = current_evaluation;

        for k in 0..self.max_iterations {
            let t = temperature(1.0f64 - (k + 1) as f64 / (self.max_iterations as f64));

            let random_neighbour = QuboSolution({
                let mut x = current_solution.0.clone();

                let x_i = self.rng.gen_range(0..x.len());

                x[x_i] = if x[x_i] == 0 { 1 } else { 0 };

                x
            });

            let random_evaluation = qubo_problem.evaluate(&random_neighbour);

            if random_evaluation < best_evaluation {
                best_solution = random_neighbour.clone();
                best_evaluation = random_evaluation;
            }

            if acceptance_probability(current_evaluation, random_evaluation, t) > self.rng.gen_range(0f64..1f64) {
                current_solution = random_neighbour;
                current_evaluation = random_evaluation;
            }

            trace!("Current evaluation is {current_evaluation}");
        }

        debug!("Final Evaluation is {best_evaluation}");

        best_solution
    }
}

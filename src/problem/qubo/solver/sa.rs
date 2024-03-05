use log::{trace};

use rand::rngs::ThreadRng;
use rand::thread_rng;
use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::qubo::solver::QuboSolver;

pub struct SimulatedAnnealer<Rng> where Rng : rand::Rng {
    rng: Rng
}

impl <Rng> SimulatedAnnealer<Rng> where Rng : rand::Rng {
    pub fn new_with_rng(rng: Rng) -> Self {
        Self {
            rng
        }
    }
}

impl SimulatedAnnealer<ThreadRng> {
    pub fn new_with_thread_rng() -> Self {
        Self::new_with_rng(thread_rng())
    }
}

impl <Rng> QuboSolver for SimulatedAnnealer<Rng> where Rng : rand::Rng {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        let mut current_solution = QuboSolution(self.rng.gen());

        let evaluation = qubo_problem.evaluate(&current_solution);

        trace!("Current Evaluation is {evaluation}");

        current_solution
    }
}

use crate::problem::qubo::{QuboProblem, QuboSolution};
use crate::problem::qubo::solver::QuboSolver;

pub struct ParallelExhaustiveSearch {
}

impl QuboSolver for ParallelExhaustiveSearch {
    fn solve(&mut self, qubo_problem: QuboProblem) -> QuboSolution {
        // qubo_problem.evaluate()
        todo!()
    }
}
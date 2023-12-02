pub mod sat;
pub mod qubo;
pub mod reductions;

use std::fmt::Debug;

use log::debug;

pub trait Problem<SolutionType, EvaluationType>: Debug {
    fn solve(&self) -> SolutionType;
    fn validate_solution(&self, solution: &SolutionType) -> EvaluationType;
}
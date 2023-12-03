pub mod sat;
pub mod qubo;
pub mod reductions;

pub trait Problem<SolutionType, EvaluationType> {
    /// Generates a solution for the problem
    fn solve(&self) -> SolutionType;
    /// Evaluates a given solution for the problem
    fn evaluate_solution(&self, solution: &SolutionType) -> EvaluationType;
}
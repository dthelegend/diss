pub mod sat;
pub mod qubo;
pub mod reductions;

pub trait Problem<SolutionType> {
    /// Generates a solution for the problem
    fn solve(&self) -> SolutionType;
}
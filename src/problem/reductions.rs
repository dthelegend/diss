use super::Problem;

pub mod sat_to_qubo;
pub mod ksat_to_qubo;
pub mod ksat_to_threesat;

pub trait Reduction<TSolutionType, TProblem: Problem<TSolutionType>, USolutionType, UProblem: Problem<USolutionType>> {
    /// Applies the reduction to transform the input problem into the output problem
    fn reduce_problem(&self, problem: TProblem) -> (UProblem, Box<dyn SolutionReductionReverser<TSolutionType, TProblem, USolutionType, UProblem>>);
}

pub trait SolutionReductionReverser<TSolutionType, TProblem: Problem<TSolutionType>, USolutionType, UProblem: Problem<USolutionType>> {
    /// Reverses the reduction for the solution
    /// # Panics
    /// May panic if the reduction is irreversible
    fn reverse_reduce_solution(&self, solution: USolutionType) -> TSolutionType;
}

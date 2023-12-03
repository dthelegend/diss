use super::Problem;

pub mod sat_to_qubo;
pub mod ksat_to_qubo;
pub mod ksat_to_threesat;

pub trait Reduction<TSolutionType, TProblem: Problem<TSolutionType>, USolutionType, UProblem: Problem<USolutionType>> {
    /// Applies the reduction to transform the input problem into the output problem
    fn reduce_problem(&self, problem: &TProblem) -> UProblem;
}

pub trait SolutionReversibleReduction<TSolutionType, TProblem: Problem<TSolutionType>, USolutionType, UProblem: Problem<USolutionType>> : Reduction<TSolutionType, TProblem, USolutionType, UProblem> {
    /// 
    fn reverse_reduce_solution(&self, problem: &TProblem, solution: USolutionType) -> TSolutionType;
}

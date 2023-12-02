use super::Problem;

pub mod sat;
pub mod ksat_to_qubo;
pub mod ksat_to_threesat;

pub trait Reducer<TSolutionType, TEvaluationType, USolutionType, UEvaluationType> {
    fn reduce(self, problem: &dyn Problem<TSolutionType, TEvaluationType>) -> Box<dyn ReducedProblem<TSolutionType, TEvaluationType, USolutionType, UEvaluationType>>;
}

pub trait ReducedProblem<TSolutionType, TEvaluationType, USolutionType, UEvaluationType>: Problem<TSolutionType, TEvaluationType> {
    fn get_reduced_problem(&self) -> &dyn Problem<USolutionType, UEvaluationType>;
    fn convert_solution(&self, solution : USolutionType) -> TSolutionType;
}
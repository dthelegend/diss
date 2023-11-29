pub mod sat;
pub mod qubo;
pub mod reductions;

pub trait Problem<SolutionType, EvaluationType> {
    fn solve(&self) -> SolutionType;
    fn validate_solution(&self, solution: &SolutionType) -> EvaluationType;
}

pub trait ReducibleProblem<T, TSolutionType, TEvaluationType, USolutionType, UEvaluationType>: Problem<TSolutionType, TEvaluationType> {
    fn solve_with_reduction(&self, reduction: T) -> TSolutionType {
        self.convert_solution(self.reduce(reduction).solve())
    }
    fn reduce(&self, reduction: T) -> Box<dyn Problem<USolutionType, UEvaluationType>>;
    fn convert_solution(&self, solution : USolutionType) -> TSolutionType;
}
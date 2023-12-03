use super::Problem;

pub mod sat_to_qubo;
pub mod ksat_to_qubo;
pub mod ksat_to_threesat;

pub trait Reducer<TSolutionType, TEvaluationType, T, USolutionType, UEvaluationType, U>
where   T: Problem<TSolutionType, TEvaluationType> + Sized,
        U: Problem<USolutionType, UEvaluationType> + Sized {
    fn reduce(self, problem: &T) -> Box<dyn ReducedProblem<TSolutionType, TEvaluationType, T, USolutionType, UEvaluationType, U> + '_>;
}

pub trait ReducedProblem<TSolutionType, TEvaluationType, T, USolutionType, UEvaluationType, U>
where   T: Problem<TSolutionType, TEvaluationType> + Sized,
        U: Problem<USolutionType, UEvaluationType> + Sized  {
    fn get_reduced_problem(&self) -> &U;
    fn get_original_problem(&self) -> &T;
    fn convert_solution(&self, solution : USolutionType) -> TSolutionType;
}

impl <TSolutionType, TEvaluationType, T, USolutionType, UEvaluationType, U> Problem<TSolutionType, TEvaluationType> for dyn ReducedProblem<TSolutionType, TEvaluationType, T, USolutionType, UEvaluationType, U>
where   T: Problem<TSolutionType, TEvaluationType> + Sized,
        U: Problem<USolutionType, UEvaluationType> + Sized {
    fn solve(&self) -> TSolutionType {
        self.convert_solution(self.get_reduced_problem().solve())
    }

    fn evaluate_solution(&self, solution: &TSolutionType) -> TEvaluationType {
        self.get_original_problem().evaluate_solution(solution)
    }
}

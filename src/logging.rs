use std::fmt::Debug;
use std::time::Duration;
use log::info;

struct SolverPerformance;

pub fn log_solver_performance<T>(solution_quality: T, iteration: usize) where T: Debug {
    info!(target: "solver_performance", solution_quality:?, iteration:%; "Solution of quality {solution_quality:?} recorded at iteration step {iteration}")
}
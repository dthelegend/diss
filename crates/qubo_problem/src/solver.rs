use crate::QuboProblem;
use common::Solver;

pub trait QuboSolver = Solver<QuboProblem>;

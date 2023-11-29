use crate::sat::KSatProblem;
use crate::qubo::QUBOProblem;

pub trait Reducer<T, U> {
    fn convert(&self, problem: T) -> U;
}

enum KSatToQuboReduction {
    Choi,
    Chancellor,
    // Insert the other Reducers
}

impl Reducer<KSatProblem, QUBOProblem> for KSatToQuboReduction {
    fn convert(&self, problem: KSatProblem) -> QUBOProblem {
        match self {
            KSatToQuboReduction::Choi => todo!(),
            KSatToQuboReduction::Chancellor => todo!(),
        }
    }
}
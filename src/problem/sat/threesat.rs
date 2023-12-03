use std::fmt::Debug;

use crate::problem::Problem;

use super::{SatVariable, SatSolution};

pub struct ThreeSatProblem(pub usize, pub Vec<[SatVariable;3]>);

impl Problem<SatSolution> for ThreeSatProblem {
    fn solve(&self) -> SatSolution {
        unimplemented!()
    }
}

impl Debug for ThreeSatProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.1.iter()
            .map(|clause|
                format!("({})", clause.iter()
                    .map(|SatVariable(is_pos, number)|
                        format!("{}{}", if *is_pos {""} else {"¬"}, number))
                    .collect::<Vec<String>>()
                    .join(" + ")))
            .collect::<Vec<String>>()
            .join(" . ");
        write!(f, "ThreeSatProblem {}", x)
    }
}
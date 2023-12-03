use std::fmt::Debug;

use crate::problem::Problem;

use super::{SatVariable, SatSolution};

pub struct ThreeSatProblem {
    pub nbvars: usize,
    pub clauses: Vec<[SatVariable;3]>
}

impl Problem<SatSolution, bool> for ThreeSatProblem {
    fn solve(&self) -> SatSolution {
        unimplemented!()
    }

    fn evaluate_solution(&self, solution: &SatSolution) -> bool {
        let ThreeSatProblem { nbvars, clauses } = &self;
        let SatSolution::Sat(solution_vector) = solution else {
            return false;
        };

        assert!(*nbvars == solution_vector.len(), "Solution vector is not same size as number of clauses");
    
        clauses.iter().all(|clause| {
            clause.iter().any(|var| {
                let SatVariable(is_pos, number) = var;
                // is_pos xor solution[i]
                is_pos ^ solution_vector[*number]
            })
        })
    }
}

impl Debug for ThreeSatProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.clauses.iter()
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
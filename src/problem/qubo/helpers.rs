use crate::problem::qubo::{QuboSolution, QuboType};

// #[inline]
// pub fn not(QuboSolution(solution_vector) : &QuboSolution, k : usize) -> QuboType {
//     1 - solution_vector[k]
// }

#[inline]
pub fn sigma(QuboSolution(solution_vector): &QuboSolution, k: usize) -> QuboType {
    2 * solution_vector[k] - 1
}

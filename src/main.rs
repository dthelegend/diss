mod qubo;
mod sat;

use rand::prelude::{Rng, thread_rng};
use crate::qubo::{QUBOSolution, QUBOProblem};

const QUBO_TEST_SIZE : usize = 4;

fn test_qubo() {
    let mut rng = thread_rng();

    let mut qubo_problem = QUBOProblem::new(QUBO_TEST_SIZE);

    println!("Generating problem...");
    for i in 0..QUBO_TEST_SIZE {
        for j in 0..(i + 1) {
            qubo_problem[(i,j)] = rng.gen_range(-10..10)
        }
    }
    println!("Generated problem:\n{:?}", qubo_problem);


    for _ in 0..1 {
        let mut solution = vec![false; QUBO_TEST_SIZE];
        for s in solution.iter_mut() {
            *s = rng.gen();
        }
        
        let qubo_solution = QUBOSolution::from(solution);

        let x = qubo_problem.evaluate_solution(&qubo_solution);

        println!("Evaluation for solution {:?} is {}", qubo_solution, x)
    }
}

fn main() {
    println!("Testing QUBO");
    test_qubo();
}

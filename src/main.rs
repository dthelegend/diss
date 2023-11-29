mod qubo;
mod sat;
mod reducers;
mod backends;

use std::{io::{self}, fs::File, path::PathBuf};
use rand::prelude::{Rng, thread_rng};
use clap::Parser;
use sat::KSatProblem;

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

fn test_sat(problem: &KSatProblem) {
    let mut rng = thread_rng();

    for _ in 0..100 {
        let mut solution = vec![false; problem.0];
        for s in solution.iter_mut() {
            *s = rng.gen();
        }
        
        let sat_solution = sat::KSATSolution::Sat(solution);

        let x = problem.validate_solution(&sat_solution);

        println!("Evaluation for solution {:?} is {}", sat_solution, x)
    }
}

#[derive(Parser)]
struct SolverCli {
    #[arg()]
    file: Option<PathBuf>
}

fn main() {
    let args = SolverCli::parse();

    let file: Box<dyn io::Read> = match args.file {
        Some(x) => Box::new(File::open(x).unwrap()),
        None => Box::new(io::stdin())
    };

    let buf_reader = io::BufReader::new(file);

    let problem = sat::KSatProblem::from_benchmark_file(buf_reader).unwrap();

    println!("{:?}", problem);
    
    test_sat(&problem);
}

mod matrix;
mod problem;
mod error;

use std::{io::{self}, fs::File, path::PathBuf};
use rand::prelude::{Rng, thread_rng};
use clap::Parser;
use problem::{Problem, sat::{KSatProblem, KSatSolution}};

fn test_sat(problem: &KSatProblem) {
    let mut rng = thread_rng();

    for _ in 0..100 {
        let mut solution = vec![false; problem.0];
        for s in solution.iter_mut() {
            *s = rng.gen();
        }
        
        let sat_solution = KSatSolution::Sat(solution);

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

    let problem = KSatProblem::from_benchmark_file(buf_reader).unwrap();

    println!("{:?}", problem);
    
    test_sat(&problem);
}

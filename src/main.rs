mod matrix;
mod problem;
mod error;

use std::{io::{self}, fs::File, path::PathBuf};
use clap::Parser;
use problem::{Problem, sat::KSatProblem, reductions::SatToQuboReduction};

use crate::problem::ReducibleProblem;

#[derive(Parser)]
struct SolverCli {
    #[arg()]
    file: Option<PathBuf>
}

fn main() -> Result<(), error::Error> {
    let args = SolverCli::parse();

    let file: Box<dyn io::Read> = match args.file {
        Some(x) => Box::new(File::open(x).unwrap()),
        None => Box::new(io::stdin())
    };

    let buf_reader = io::BufReader::new(file);

    let problem = KSatProblem::from_benchmark_file(buf_reader)?;

    println!("{:?}", problem);
    
    // let solution = problem.solve();
    let solution = problem.solve_with_reduction(SatToQuboReduction::Choi);

    if !problem.validate_solution(&solution) {
        panic!("Found invalid solution {:?}", solution);
    }
    
    println!("{}", solution);

    Ok(())
}

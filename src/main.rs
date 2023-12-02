mod matrix;
mod problem;
mod error;

use std::{io::{self}, fs::File, path::PathBuf, thread::available_parallelism};
use clap::Parser;
use log::{info, set_max_level, LevelFilter};

use crate::problem::{reductions::sat::SatToQuboReduction, sat::ksat::KSatProblem, ReducibleProblem, Problem};

#[cfg(test)]
mod tests {
    use crate::problem::Problem;

    use super::*;

    #[test]
    fn test_qubo() {
        const QUBO_TEST_SIZE : usize = 10;
        
        let mut rng = rand::thread_rng();
    
        let mut qubo_problem = problem::qubo::QuboProblem::new(QUBO_TEST_SIZE);
        
        assert_eq!(QUBO_TEST_SIZE, qubo_problem.get_size());
    
        println!("Generating problem...");
        for i in 0..QUBO_TEST_SIZE {
            for j in 0..(i + 1) {
                println!("({},{})", i, j);
                qubo_problem[(i,j)] = rand::Rng::gen_range(&mut rng, -10..11)
            }
        }
        println!("Generated problem:\n{:?}", qubo_problem);
        
        let qubo_solution = qubo_problem.solve();
    
        let x = qubo_problem.validate_solution(&qubo_solution);
    
        println!("Evaluation for solution {:?} is {}", qubo_solution, x)
    }
}

#[derive(Parser)]
struct SolverCli {
    #[arg(short='j')]
    jobs: Option<usize>,
    #[arg()]
    file: Option<PathBuf>,
    #[arg(short='v', action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), error::Error> {
    let args = SolverCli::parse();

    let verbosity = match args.verbose {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace
    };

    simple_logger::SimpleLogger::new().init().unwrap();
    set_max_level(verbosity);

    let n_jobs = match args.jobs {
        Some(x) => x,
        None => available_parallelism()
            .map_err(|x| error::Error { kind: error::ErrorKind::IO(x.kind())})?.get(),
    };

    info!("Running on {} threads", n_jobs);

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

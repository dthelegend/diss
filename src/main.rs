mod problem;

use std::{io::{self}, fs::File, path::PathBuf, time::Instant};
use std::error::Error;
use std::io::Read;
use clap::Parser;
use log::{info, set_max_level, LevelFilter, debug, log_enabled, trace, error};
use problem::sat::KSatProblem;
use crate::problem::qubo::solver::{QuboSolver, SimulatedAnnealer};
use crate::problem::sat::reducer::{Choi, QuboToSatReduction};

#[derive(Parser)]
struct SolverCli {
    /// Do not log anything; Overrides verbose
    #[arg(short='q', long="quiet")]
    quiet: bool,
    /// The file to read. If not provided it defaults to STDIN
    #[arg()]
    file: Option<PathBuf>,
    /// Logs more information
    #[arg(short='v', long="verbose", action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = SolverCli::parse();

    let verbosity = if args.quiet {
        LevelFilter::Off
    } else {
        match args.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace
        }
    };

    simple_logger::SimpleLogger::new().init().unwrap();
    set_max_level(verbosity);

    info!("Current Verbosity is {}", verbosity);

    let file: Box<dyn Read> = match args.file {
        None => {
            debug!("Reading problem from STDIN");

            Box::new(std::io::stdin())
        },
        Some(path) => {
            debug!("Reading problem from file \"{}\"", path.to_string_lossy());

            Box::new(File::open(path)?)
        }
    };

    let problem = KSatProblem::from_benchmark_file(file)?;

    debug!("Ingested problem file and produced {:?}", problem);

    let (qubo_problem, up_modeller) = Choi::reduce(&problem);

    let mut solver = SimulatedAnnealer::new_with_thread_rng();

    let qubo_solution = solver.solve(qubo_problem);

    let solution = up_modeller.up_model(qubo_solution);

    debug!("{:?}", solution);

    println!("{}", solution);

    Ok(())
}

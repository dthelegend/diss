use clap::Parser;
use common::{Reduction, Solver};
use log::{debug, error, info, set_max_level, trace, LevelFilter};
use qubo_solvers::{ ExhaustiveSearch, ParallelExhaustiveSearch };
use sat_problem::{KSatProblem, SatSolution};
use sat_to_qubo_reducers::chancellor::Chancellor;
use std::error::Error;
use std::io::Read;
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};
use sat_to_qubo_reducers::nusslein::Nusslein;

#[derive(Parser)]
struct SolverCli {
    /// Do not log anything; Overrides verbose
    #[arg(short = 'q', long = "quiet")]
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
            _ => LevelFilter::Trace,
        }
    };

    simple_logger::SimpleLogger::new().init()?;
    set_max_level(verbosity);

    info!("Current Verbosity is {}", verbosity);

    let problem = {
        let file: Box<dyn Read> = match args.file {
            None => {
                debug!("Reading problem from STDIN");

                Box::new(io::stdin())
            }
            Some(path) => {
                debug!("Reading problem from file \"{}\"", path.to_string_lossy());

                Box::new(File::open(path)?)
            }
        };

        KSatProblem::from_benchmark_file(file)?
    };

    trace!("Ingested problem {:?}", problem);

    let (qubo_problem, up_modeller) = {
        // Choi::reduce(&problem)
        // Chancellor::reduce(&problem)
        Nusslein::reduce(&problem)
    };

    debug!("Reduced problem size is {}", qubo_problem.get_size());
    trace!("Reduced problem produced {:?}", qubo_problem);

    let mut solver = {
        // TODO Allow this to be set by CLI arg
        // SimulatedAnnealer::new_with_thread_rng(1_000)
        // ExhaustiveSearch::new()
        // ParallelExhaustiveSearch::new(5)
        // ParallelExhaustiveSearch::with_cuda(22)
        ParallelExhaustiveSearch::with_cuda(11)
        // Mopso::new_with_thread_rng(1024, 1024)
    };

    let qubo_solution = solver.solve(&qubo_problem);

    let mut solution = up_modeller.up_model(qubo_solution);

    debug!("{:?}", solution);

    if !problem.evaluate(&solution) {
        error!("Solution that was generated does not satisfy the problem!");
        solution = SatSolution::Unknown
    }

    println!("{}", solution);

    Ok(())
}

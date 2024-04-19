use clap::{self, Parser, ValueEnum};
use common::{Reduction, data_recorder::DataRecorder, Solver};
use log::{debug, error, info, set_max_level, trace, LevelFilter};
use qubo_problem::{QuboProblem, QuboSolution, solver::QuboSolver};
use qubo_solvers::{ExhaustiveSearch, MomentumAnnealer, ParallelExhaustiveSearch, SimulatedAnnealer};
use sat_problem::{KSatProblem, SatSolution};
use sat_to_qubo_reducers::chancellor::Chancellor;
use sat_to_qubo_reducers::nusslein::Nusslein;
use sat_to_qubo_reducers::nusslein23::Nusslein23;
use std::error::Error;
use std::io::Read;
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};
use common::data_recorder::CsvDataRecorder;
use sat_to_qubo_reducers::choi::Choi;

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Solvers {
    SimulatedAnnealing,
    ExhaustiveSearch,
    ParallelExhaustiveSearch,
    MomentumAnnealing
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Reducers {
    Chancellor,
    Choi,
    Nusslein,
    Nusslein23
}

impl Solver<QuboProblem> for Solvers {
    fn solve(&mut self, qubo_problem: &QuboProblem, mut logger: Option<impl DataRecorder>) -> QuboSolution {
        match self {
            Self::SimulatedAnnealing => SimulatedAnnealer::new_with_thread_rng(1_000).solve(qubo_problem, logger),
            Self::ExhaustiveSearch => ExhaustiveSearch::new().solve(qubo_problem, logger),
            Self::ParallelExhaustiveSearch => ParallelExhaustiveSearch::new(3).solve(qubo_problem, logger),
            Self::MomentumAnnealing => MomentumAnnealer::new(1_000).solve(qubo_problem, logger)
        }
    }
}

#[derive(Parser)]
struct SolverCli {
    /// Do not log anything; Overrides verbose
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
    // The file to log solver steps to in csv format
    #[arg(short = 'l', long = "log")]
    log_file: Option<PathBuf>,
    /// A standard DIMACS CNF file to read. If not provided it will attempt to read a CNF file from the STDIN
    #[arg()]
    file: Option<PathBuf>,
    /// Logs more information about the program. Repeat to increase verbosity
    #[arg(short='v', long="verbose", action = clap::ArgAction::Count)]
    verbose: u8,
    // The reducer to use (Not Functional)
    #[arg(value_enum, long="reducer", default_value_t=Reducers::Chancellor)]
    reducer: Reducers,
    // The solver to use
    #[arg(value_enum, long="solver", default_value_t=Solvers::ParallelExhaustiveSearch)]
    solver: Solvers
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
    
    let logger = args.log_file.map(|x| {
        CsvDataRecorder::new_from_path(x)
    }).transpose()?;

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
        // Chancellor::reduce(&problem)
        // Choi::reduce(&problem)
        // Nusslein::reduce(&problem)
        Nusslein23::reduce(&problem)
    };

    debug!("Reduced problem size is {}", qubo_problem.get_size());
    trace!("Reduced problem produced {:?}", qubo_problem);

    let mut solver = {
        args.solver
    };

    let qubo_solution = solver.solve(&qubo_problem, logger);

    let mut solution = up_modeller.up_model(qubo_solution);

    debug!("{:?}", solution);

    if !problem.evaluate(&solution) {
        error!("Solution that was generated does not satisfy the problem!");
        solution = SatSolution::Unknown
    }

    println!("{}", solution);

    Ok(())
}

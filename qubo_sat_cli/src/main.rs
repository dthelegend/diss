use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};
use std::error::Error;
use std::io::Read;
use std::num::{NonZero, NonZeroUsize};

use clap::{self, Parser, ValueEnum};
use log::{debug, error, info, LevelFilter, set_max_level, trace};

use optimizers::core::{Reduction, Solver};
use optimizers::qubo::{QuboProblem, QuboSolution};
use optimizers::qubo::solvers::{ExhaustiveSearch, MomentumAnnealer, Mopso, ParallelExhaustiveSearch, SimulatedAnnealer};
use optimizers::sat::{KSatProblem, SatSolution};
use optimizers::sat::reductions::qubo::chancellor::Chancellor;
use optimizers::sat::reductions::qubo::choi::Choi;
use optimizers::sat::reductions::qubo::nusslein23::Nusslein23;
use optimizers::sat::reductions::qubo::nusslein::Nusslein;

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum SolverOptions {
    SimulatedAnnealing,
    ExhaustiveSearch,
    ParallelExhaustiveSearch,
    MomentumAnnealing,
    Mopso
}

impl Solver<QuboProblem> for SolverOptions {
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        match self {
            Self::SimulatedAnnealing => SimulatedAnnealer::new(NonZero::new(1_000).unwrap(), std::thread::available_parallelism().unwrap()).solve(qubo_problem),
            Self::ExhaustiveSearch => ExhaustiveSearch::new().solve(qubo_problem),
            Self::ParallelExhaustiveSearch => ParallelExhaustiveSearch::new(NonZeroUsize::new((usize::BITS as usize) - std::thread::available_parallelism().unwrap().get()).unwrap()).solve(qubo_problem),
            Self::MomentumAnnealing => MomentumAnnealer::new(1_000).solve(qubo_problem),
            Self::Mopso => Mopso::new().solve(qubo_problem)
        }
    }
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum ReducerOptions {
    Chancellor,
    Choi,
    Nusslein,
    Nusslein23,
}

enum ReducerWrapper {
    Chancellor(Chancellor),
    Choi(Choi),
    Nusslein(Nusslein),
    Nusslein23(Nusslein23),
}

impl ReducerWrapper {
    fn new(option: ReducerOptions, ksat_problem: &KSatProblem) -> (QuboProblem, Self) {
        match option {
            ReducerOptions::Chancellor => {
                let (q, r) = Chancellor::reduce(ksat_problem);

                (q, Self::Chancellor(r))
            }
            ReducerOptions::Choi => {
                let (q, r) = Choi::reduce(ksat_problem);

                (q, Self::Choi(r))
            }
            ReducerOptions::Nusslein => {
                let (q, r) = Nusslein::reduce(ksat_problem);

                (q, Self::Nusslein(r))
            }
            ReducerOptions::Nusslein23 => {
                let (q, r) = Nusslein23::reduce(ksat_problem);

                (q, Self::Nusslein23(r))
            }
        }
    }

    fn up_model(self, solution: QuboSolution) -> SatSolution {
        match self {
            ReducerWrapper::Chancellor(r) => r.up_model(solution),
            ReducerWrapper::Choi(r) => r.up_model(solution),
            ReducerWrapper::Nusslein(r) => r.up_model(solution),
            ReducerWrapper::Nusslein23(r) => r.up_model(solution),
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
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,
    // The reducer to use
    #[arg(value_enum, long = "reducer", default_value_t = ReducerOptions::Chancellor)]
    reducer: ReducerOptions,
    // The solver to use
    #[arg(value_enum, long = "solver", default_value_t = SolverOptions::ParallelExhaustiveSearch)]
    solver: SolverOptions,
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
        ReducerWrapper::new(args.reducer, &problem)
    };

    debug!("Reduced problem size is {}", qubo_problem.get_size());
    trace!("Reduced problem produced {:?}", qubo_problem);

    let mut solver = {
        args.solver
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

use std::fmt::Debug;
use std::io;
use std::num;
use regex::Regex;
use crate::reducers::Reducer;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

pub enum ErrorKind {
    IO(io::ErrorKind),
    ParseError(num::IntErrorKind),
    ClauseError,
    HeaderError,
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(arg0) => f.debug_tuple("IO").field(arg0).finish(),
            Self::ParseError(arg0) => f.debug_tuple("ParseError").field(arg0).finish(),
            Self::ClauseError => write!(f, "ClauseError"),
            Self::HeaderError => write!(f, "HeaderError"),
        }
    }
}

// Whether the variable is negated and Index of the variable in the solution
pub struct KSatVariable(pub bool, pub usize);

impl Debug for KSatVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KSatVariable {}{}", if self.0 {""} else {"¬"}, self.1)
    }
}

pub enum KSATSolution {
    Sat(Vec<bool>),
    Unsat
}

impl Debug for KSATSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(arg0) => write!(f, "Sat {}", arg0.iter().enumerate().map(|(i, x)| format!("{}{}", if *x {""} else {"¬"}, i)).collect::<Vec<String>>().join(" ")),
            Self::Unsat => write!(f, "Unsat"),
        }
    }
}

// Conjunctive normal form KSAT Problem with N Variables
pub struct KSatProblem(pub usize, Vec<Vec<KSatVariable>>);

impl Debug for KSatProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.1.iter()
            .map(|clause|
                format!("({})", clause.iter()
                    .map(|KSatVariable(is_pos, number)|
                        format!("{}{}", if *is_pos {""} else {"¬"}, number))
                    .collect::<Vec<String>>()
                    .join(" + ")))
            .collect::<Vec<String>>()
            .join(" . ");
        write!(f, "KSatProblem {}", x)
    }
}

impl KSatProblem {
    pub fn from_benchmark_file(file: impl io::BufRead) -> Result<Self, Error> {
        let mut line_result_iterator = file.lines();
        let mut next_line: String;

        while {
            let next_line_result = line_result_iterator.next();
            next_line = next_line_result.expect("File missing instance header")
                .map_err(|e| Error {kind: ErrorKind::IO(e.kind())})?;

            next_line.starts_with('c')
        }  {
            // Consume beginning comment lines
        }

        // next_line should now be the instance header
        let benchmark_header_re = Regex::new(r"^p cnf (?<nbvar>\d+) (?<nbclauses>\d+)$")
            .expect("Failed to compile the benchmark header RegEx!");
        let captures = benchmark_header_re.captures(next_line.as_str())
            .ok_or(Error { kind: ErrorKind::HeaderError })?;

        let nbclauses: usize = captures["nbclauses"].parse()
            .map_err(|e: num::ParseIntError| Error { kind: ErrorKind::ParseError(e.kind().clone())})?;
        let nbvar: usize = captures["nbvar"].parse()
            .map_err(|e: num::ParseIntError| Error { kind: ErrorKind::ParseError(e.kind().clone())})?;

        let mut clauses = Vec::with_capacity(nbclauses);

        for line_result in line_result_iterator.take(nbclauses) {
            next_line = line_result
                .map_err(|e| Error {kind: ErrorKind::IO(e.kind())})?;

            let mut clause_line: Vec<isize> = next_line
                .split(' ')
                .map(|x| x.parse::<isize>().unwrap_or_else(|_| panic!("Unexpected non integer found while parsing clause: {}", x)))
                .collect();

            let end_number = clause_line.pop().expect("Clause line is empty!");
            assert!(end_number == 0, "Clause line incorrectly ends with somethind other than 0: {}", end_number);
            assert!(clause_line.iter().all(|x| *x != 0), "Clause line contains values of 0: {:?}", clause_line);

            let clause : Vec<KSatVariable> = clause_line.iter()
                .map(|x| {
                    KSatVariable(x.signum() > 0, x.unsigned_abs() - 1)
                })
                .collect();

            if clause.iter().any(|f| f.1 >= nbclauses) {
                return Err(Error { kind: ErrorKind::ClauseError });
            }
            
            clauses.push(clause);
        }

        Ok(KSatProblem(nbvar, clauses))
    }

    pub fn validate_solution(&self, solution: &KSATSolution) -> bool {
        let KSatProblem(nbvars, clauses) = &self;
        let KSATSolution::Sat(solution_vector) = solution else {
            return false;
        };

        assert!(*nbvars == solution_vector.len(), "Solution vector is not same size as number of clauses");
        
        clauses.iter().all(|clause| {
            clause.iter().any(|var| {
                let KSatVariable(is_pos, number) = var;
                // is_pos xor solution[i]
                is_pos ^ solution_vector[*number]
            })
        })
    }

    pub fn reduce<T>(self, reducer: impl Reducer<Self, T>) -> T {
        reducer.convert(self)
    }

    pub fn find_solution(&self, backend: &dyn KSATSolutionBackend) -> KSATSolution {
        backend.find_solution(self)
    }
}

pub trait KSATSolutionBackend {
    fn find_solution(&self, solution: &KSatProblem) -> KSATSolution;
}
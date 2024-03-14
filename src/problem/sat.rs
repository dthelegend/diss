pub mod reducer;

use log::{debug, trace};
use nalgebra::DVector;
use regex::Regex;
use std::fmt::Display;
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Clone)]
pub enum SatSolution {
    Sat(DVector<bool>),
    Unsat,
    Unknown,
}

// Conjunctive normal form KSAT Problem with N Variables
#[derive(Clone, Copy)]
pub struct SatVariable(pub bool, pub usize);

impl Debug for SatVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KSatVariable {}{}",
            if self.0 { "" } else { "¬" },
            self.1
        )
    }
}

impl Debug for SatSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(arg0) => write!(
                f,
                "Sat ({})",
                arg0.iter()
                    .enumerate()
                    .map(|(i, x)| format!("{}{}", if *x { "" } else { "¬" }, i))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Self::Unsat => write!(f, "Unsat"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Display for SatSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(_) => write!(f, "SAT"),
            Self::Unsat => write!(f, "UNSAT"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

pub struct KSatProblem {
    pub nb_vars: usize,
    pub clause_list: Vec<Vec<SatVariable>>,
}

#[derive(Error, Debug)]
pub enum KSatProblemError {
    #[error("Error reading the file")]
    FileError(#[from] std::io::Error),
    #[error("Incorrect File Header")]
    InvalidHeader,
    #[error("Invalid variable declared in file")]
    InvalidVariable,
}

impl KSatProblem {
    pub fn from_benchmark_file(file: impl Read) -> Result<Self, KSatProblemError> {
        let buffered_file = BufReader::new(file);
        let mut line_result_iterator = buffered_file.lines();
        let mut next_line: String;

        while {
            let next_line_result = line_result_iterator.next();
            next_line = next_line_result
                .expect("File missing instance header")
                .map_err(KSatProblemError::FileError)?;

            next_line.starts_with('c')
        } {
            // Consume beginning comment lines
        }

        // next_line should now be the instance header
        let benchmark_header_re = Regex::new(r"^p cnf (?<nb_var>\d+) (?<nb_clauses>\d+)$")
            .expect("Failed to compile the benchmark header RegEx!");
        let captures = benchmark_header_re
            .captures(next_line.as_str())
            .ok_or(KSatProblemError::InvalidHeader)?;

        let nb_clauses: usize = captures["nb_clauses"]
            .parse()
            .or(Err(KSatProblemError::InvalidHeader))?;
        let nb_var: usize = captures["nb_var"]
            .parse()
            .or(Err(KSatProblemError::InvalidHeader))?;

        let mut clauses = Vec::with_capacity(nb_clauses);

        for line_result in line_result_iterator.take(nb_clauses) {
            next_line = line_result.map_err(KSatProblemError::FileError)?;

            let mut clause_line: Vec<isize> = next_line
                .split_whitespace()
                .map(|x| {
                    x.parse::<isize>().unwrap_or_else(|_| panic!("Unexpected non integer found while parsing clause: {}", x))
                })
                .collect();

            let end_number = clause_line.pop().expect("Clause line is empty!");
            assert_eq!(
                end_number, 0,
                "Clause line incorrectly ends with something other than 0: {}",
                end_number
            );
            assert!(
                clause_line.iter().all(|x| *x != 0),
                "Clause line contains values of 0: {:?}",
                clause_line
            );

            let clause: Vec<SatVariable> = clause_line
                .iter()
                .map(|x| SatVariable(x.signum() > 0, x.unsigned_abs() - 1))
                .collect();

            if clause.iter().any(|f| f.1 >= nb_var) {
                return Err(KSatProblemError::InvalidVariable);
            }

            clauses.push(clause);
        }

        Ok(KSatProblem {
            nb_vars: nb_var,
            clause_list: clauses,
        })
    }

    pub fn evaluate(&self, solution: &SatSolution) -> bool {
        let KSatProblem {
            nb_vars,
            clause_list: clauses,
        } = &self;
        let SatSolution::Sat(solution_vector) = solution else {
            // Solutions other than SAT are considered to be invalid solutions for a problem for simplicity
            return true;
        };

        assert_eq!(
            *nb_vars,
            solution_vector.len(),
            "Solution vector is not same size as number of clauses"
        );

        clauses.iter().all(|clause| {
            let x = clause
                .iter()
                .any(|&SatVariable(is_pos, number)| !(is_pos ^ solution_vector[number]));

            if !x {
                debug!("Clause violated! {:?}", clause)
            } else {
                trace!("Verified clause {:?}", clause)
            }

            x
        })
    }
}

impl Debug for KSatProblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self
            .clause_list
            .iter()
            .map(|clause| {
                format!(
                    "({})",
                    clause
                        .iter()
                        .map(|SatVariable(is_pos, number)| format!(
                            "{}{}",
                            if *is_pos { "" } else { "¬" },
                            number
                        ))
                        .collect::<Vec<String>>()
                        .join(" + ")
                )
            })
            .collect::<Vec<String>>()
            .join(" . ");
        write!(f, "KSatProblem {}", x)
    }
}

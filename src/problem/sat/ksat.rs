use std::{io, num, fmt::Debug};
use log::{log_enabled, debug};
use regex::Regex;

use crate::{error::{Error, ErrorKind}, problem::Problem};
use super::{SatSolution, SatVariable};

#[derive(Clone)]
pub struct KSatProblem(pub usize, pub Vec<Vec<SatVariable>>);

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

            let clause : Vec<SatVariable> = clause_line.iter()
                .map(|x| {
                    SatVariable(x.signum() > 0, x.unsigned_abs() - 1)
                })
                .collect();

            if clause.iter().any(|f| f.1 >= nbvar) {
                return Err(Error { kind: ErrorKind::ClauseError });
            }
        
            clauses.push(clause);
        }

        Ok(KSatProblem(nbvar, clauses))
    }

    pub fn evaluate_solution(&self, solution: &SatSolution) -> bool {
        let KSatProblem(nbvars, clauses) = &self;
        let SatSolution::Sat(solution_vector) = solution else {
            // Solutions other than SAT are considered to be valid solutions for a problem for simplicity
            return true;
        };

        assert!(*nbvars == solution_vector.len(), "Solution vector is not same size as number of clauses");
    
        clauses.iter().all(|clause| {
            let x = clause.iter().any(|&SatVariable(is_pos, number)| {
               !(is_pos ^ solution_vector[number])
            });

            if !x {
                debug!("Clause violated! {:?}", clause)
            }

            x
        })
    }
}

impl Problem<SatSolution> for KSatProblem {
    fn solve(&self) -> SatSolution {
        unimplemented!()
    }
}

impl Debug for KSatProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.1.iter()
            .map(|clause|
                format!("({})", clause.iter()
                    .map(|SatVariable(is_pos, number)|
                        format!("{}{}", if *is_pos {""} else {"¬"}, number))
                    .collect::<Vec<String>>()
                    .join(" + ")))
            .collect::<Vec<String>>()
            .join(" . ");
        write!(f, "KSatProblem {}", x)
    }
}

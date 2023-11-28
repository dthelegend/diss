use std::io;
use regex::Regex;

pub struct KSatVariable {
    number: usize,
    is_neg: bool
}

// Conjunctive normal form KSAST Problem
pub struct KSatProblem {
    clauses: Vec<Vec<KSatVariable>>
}

impl KSatProblem {
    fn from_benchmark_file(file: impl io::BufRead) -> Result<Self, io::Error> {
        let mut line_result_iterator = file.lines();
        let mut next_line: String;

        while {
            let next_line_result = line_result_iterator.next();
            next_line = next_line_result.expect("File missing instance header")?;

            next_line.starts_with("c")
        }  {
            // Consume beginning comment lines
        }

        // next_line should now be the instance header
        let benchmark_header_re = Regex::new(r"^p cnf (?<nbvar>\d+) (?<nbclauses>\d+)$")
            .expect("Failed to compile the benchmark header RegEx!");
        let captures = benchmark_header_re.captures(next_line.as_str())
            .expect("File missing instance header");

        let nbclauses: usize = captures["nbclauses"].parse().unwrap();
        let nbvar: usize = captures["nbvar"].parse().unwrap();

        let mut clauses = Vec::with_capacity(nbclauses);

        for line_result in line_result_iterator.take(nbvar) {
            next_line = line_result?;
            
            let clause = next_line.split(" ")
                .map(|x| x.parse::<isize>().unwrap())
                .map(|x| KSatVariable {
                    is_neg: x.signum() > 0,
                    number: x.abs() as usize
                })
                .collect();

            clauses.push(clause);
        }

        Ok(KSatProblem { clauses })
    }
}
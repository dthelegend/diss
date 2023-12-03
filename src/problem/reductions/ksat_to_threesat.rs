use std::iter::zip;

use crate::problem::sat::{threesat::ThreeSatProblem, ksat::KSatProblem, SatSolution, SatVariable};

use super::{Reducer, ReducedProblem};

pub struct KSatToThreeSatReducer;

pub struct KSatToThreeSatReducedProblem<'a> {
    problem: &'a KSatProblem,
    reduced_problem: ThreeSatProblem
}

impl Reducer<SatSolution, bool, KSatProblem, SatSolution, bool, ThreeSatProblem> for KSatToThreeSatReducer {
    fn reduce(self, problem: &KSatProblem) -> Box<dyn super::ReducedProblem<SatSolution, bool, KSatProblem, SatSolution, bool, ThreeSatProblem> + '_> {
        let KSatProblem(size, clauses) = problem;

        let mut reduced_problem = ThreeSatProblem { nbvars: *size, clauses: Vec::with_capacity(*size) };

        for clause in clauses {
            match &clause[..] {
                [] => panic!("Empty Clause found during reduction"),
                [l] => {
                    let z1 =  reduced_problem.nbvars;
                    reduced_problem.nbvars += 1;
                    let z2 =  reduced_problem.nbvars;
                    reduced_problem.nbvars += 1;
                
                    reduced_problem.clauses.push([
                        *l,
                        SatVariable(true, z1),
                        SatVariable(true, z2)
                    ]);
                    reduced_problem.clauses.push([
                            *l,
                            SatVariable(false, z1),
                            SatVariable(true, z2)
                    ]);
                    reduced_problem.clauses.push([
                            *l,
                            SatVariable(true, z1),
                            SatVariable(false, z2)
                    ]);
                    reduced_problem.clauses.push([
                            *l,
                            SatVariable(false, z1),
                            SatVariable(false, z2)
                    ]);
                },
                [l1, l2] => {
                    let z1 = reduced_problem.nbvars;
                    reduced_problem.nbvars += 1;
                
                    reduced_problem.clauses.push([
                        *l1,
                        *l2,
                        SatVariable(true, z1)
                    ]);
                    reduced_problem.clauses.push([
                        *l1,
                        *l2,
                        SatVariable(true, z1)
                    ]);
                },
                [l1,l2,l3] => reduced_problem.clauses.push([*l1,*l2,*l3]),
                ln => {
                    let zn : Vec<usize> = (0..(ln.len() - 3)).map(|x| x + reduced_problem.nbvars).collect();
                    reduced_problem.nbvars += ln.len() - 3;
                
                    reduced_problem.clauses.push([
                        ln[0],
                        ln[1],
                        SatVariable(true, zn[0])
                    ]);
                
                    for (li, (zi, zj)) in zip(ln[2..ln.len() - 2].iter(), zip(zn[0..zn.len() - 2].iter(), zn[0..zn.len() - 2].iter())) {
                        reduced_problem.clauses.push([
                            *li,
                            SatVariable(false, zn[*zi]),
                            SatVariable(true, zn[*zj])
                        ])
                    }

                    reduced_problem.clauses.push([
                        ln[ln.len() - 2],
                        ln[ln.len() - 1],
                        SatVariable(false, zn[zn.len() - 1])
                    ]);
                
                }
            }
        }

        Box::new(KSatToThreeSatReducedProblem {
            reduced_problem,
            problem
        })
    }
}

impl <'a> ReducedProblem<SatSolution, bool, KSatProblem, SatSolution, bool, ThreeSatProblem> for KSatToThreeSatReducedProblem<'a> {
    fn get_reduced_problem(&self) -> &ThreeSatProblem {
        &self.reduced_problem
    }

    fn get_original_problem(&self) -> &KSatProblem {
        &self.problem
    }

    fn convert_solution(&self, solution : SatSolution) -> SatSolution {
        match solution {
            SatSolution::Sat(x) => SatSolution::Sat(x[..self.reduced_problem.clauses.len()].to_vec()),
            _ => solution
        }
    }
}

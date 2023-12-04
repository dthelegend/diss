use std::iter::zip;

use crate::problem::sat::{threesat::ThreeSatProblem, ksat::KSatProblem, SatVariable, SatSolution};

use super::*;

pub struct KSatToThreeSatReduction;

pub struct KSatToThreeSatSolutionReductionReverser(usize);

impl Reduction<SatSolution, KSatProblem, SatSolution, ThreeSatProblem> for KSatToThreeSatReduction {
    fn reduce_problem(&self, problem: KSatProblem) -> (ThreeSatProblem, Box<dyn SolutionReductionReverser<SatSolution, KSatProblem, SatSolution, ThreeSatProblem>>) {
        let KSatProblem(size, clauses) = problem;

        let mut reduced_problem = ThreeSatProblem(size, Vec::with_capacity(size));

        for clause in clauses {
            match &clause[..] {
                [] => panic!("Empty Clause found during reduction"),
                [l] => {
                    let z1 =  reduced_problem.0;
                    reduced_problem.0 += 1;
                    let z2 =  reduced_problem.0;
                    reduced_problem.0 += 1;
                
                    reduced_problem.1.push([
                        *l,
                        SatVariable(true, z1),
                        SatVariable(true, z2)
                    ]);
                    reduced_problem.1.push([
                            *l,
                            SatVariable(false, z1),
                            SatVariable(true, z2)
                    ]);
                    reduced_problem.1.push([
                            *l,
                            SatVariable(true, z1),
                            SatVariable(false, z2)
                    ]);
                    reduced_problem.1.push([
                            *l,
                            SatVariable(false, z1),
                            SatVariable(false, z2)
                    ]);
                },
                [l1, l2] => {
                    let z1 = reduced_problem.0;
                    reduced_problem.0 += 1;
                
                    reduced_problem.1.push([
                        *l1,
                        *l2,
                        SatVariable(true, z1)
                    ]);
                    reduced_problem.1.push([
                        *l1,
                        *l2,
                        SatVariable(false, z1)
                    ]);
                },
                [l1,l2,l3] => reduced_problem.1.push([*l1,*l2,*l3]),
                ln => {
                    let zn : Vec<usize> = (0..(ln.len() - 3)).map(|x| x + reduced_problem.0).collect();
                    reduced_problem.0 += ln.len() - 3;
                
                    reduced_problem.1.push([
                        ln[0],
                        ln[1],
                        SatVariable(true, zn[0])
                    ]);
                
                    for (li, (zi, zj)) in zip(ln[2..ln.len() - 2].iter(), zip(zn[0..zn.len() - 2].iter(), zn[0..zn.len() - 2].iter())) {
                        reduced_problem.1.push([
                            *li,
                            SatVariable(false, zn[*zi]),
                            SatVariable(true, zn[*zj])
                        ])
                    }

                    reduced_problem.1.push([
                        ln[ln.len() - 2],
                        ln[ln.len() - 1],
                        SatVariable(false, zn[zn.len() - 1])
                    ]);
                
                }
            }
        }

        (reduced_problem, Box::new(KSatToThreeSatSolutionReductionReverser(size)))
    }
}

impl SolutionReductionReverser<SatSolution, KSatProblem, SatSolution, ThreeSatProblem> for KSatToThreeSatSolutionReductionReverser {
    fn reverse_reduce_solution(&self, solution: SatSolution) -> SatSolution {
        let KSatToThreeSatSolutionReductionReverser(og_size) = self;
        
        match solution {
            SatSolution::Sat(sv) => SatSolution::Sat(sv[..*og_size].to_vec()),
            _ => solution
        }
    }
}

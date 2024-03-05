use log::debug;

use crate::problem::{sat::{SatSolution, ksat::KSatProblem, threesat::ThreeSatProblem, SatVariable}, qubo::{QuboProblem, QuboSolution}};

use super::{*, sat_to_qubo::ThreeSatToQuboReduction, ksat_to_threesat::KSatToThreeSatReduction};

#[derive(Default)]
pub enum KSatToQuboReduction {
    /// An optimised reduction of K-SAT to 3-SAT to MIS to QUBO <br/>
    /// Not expecting this to run well :/ <br />
    /// Reduction happens in O(n^2)
    Choi,
    /// An optimised reduction from 3 SAT to Max-2-SAT to QUBO <br/>
    /// Also not expecting this to run well :/
    Novel,
    /// The current state-of-the-art reduction
    #[default]
    Chancellor,
    /// A reduction that scales well for sub-quadratic QUBO formulations <br/>
    /// You cannot tell looking at a problem whether |E| = O(k|V|) or |E| = O(|V|*|V|)
    Nuesslein2022,
    /// A reduction that scales better than Chancellor, but the paper on it is still in preprint
    Nuesslein2023,
}

pub enum KSatToQuboSolutionReductionReverser {
    Choi {
        threesat_reverser: Box<dyn SolutionReductionReverser<SatSolution, KSatProblem, SatSolution, ThreeSatProblem>>,
        qubo_reverser: Box<dyn SolutionReductionReverser<SatSolution, ThreeSatProblem, QuboSolution, QuboProblem>>
    },
    Novel,
    Chancellor(usize),
    Nuesslein2022,
    Nuesslein2023,
}

impl Reduction<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboReduction {
    fn reduce_problem(&self, problem: KSatProblem) -> (QuboProblem, Box<dyn SolutionReductionReverser<SatSolution, KSatProblem, QuboSolution, QuboProblem>>) {
        match self {
            KSatToQuboReduction::Choi => {
                let (threesat_problem, threesat_reverser) = KSatToThreeSatReduction.reduce_problem(problem);

                debug!("Choi reduction part 1 produced: {:?}", threesat_problem);

                let (qubo_problem, qubo_reverser) = ThreeSatToQuboReduction::Choi.reduce_problem(threesat_problem);

                debug!("Choi reduction part 2 produced: {}", qubo_problem);

                (qubo_problem, Box::new(KSatToQuboSolutionReductionReverser::Choi { threesat_reverser, qubo_reverser}))
            },
            KSatToQuboReduction::Novel => todo!(),
            KSatToQuboReduction::Chancellor => {
                let KSatProblem(nb_vars, clause_list) = problem;
                
                let num_ancillae: usize = clause_list.iter().map(|x| x.len()).sum();

                debug!("Chancellor reduction requires {} ancillae", num_ancillae);
                
                let mut q_matrix = QuboProblem::new(nb_vars + num_ancillae);
                
                // Coupling Strength in the ising model
                const J : i32 = 100;
                // Site Strength in the ising model
                const H : i32 = -J;
                // Not sure?
                const G : i32 = 1;

                let mut new_var_counter = 0;
                for clause in clause_list {
                    // \sum^k_{i=1}
                    for (i_index, i_var) in clause.iter().enumerate() {
                        // \sum^{i-1}_{j=1}
                        for (j_index, j_var) in clause[0..i_index].iter().enumerate() {
                            // J c(i) c(j)\sigma^z_i\sigma^z_j
                            let sign: i32 = if i_var.0 == j_var.0 { 1 } else { -1 };
                            q_matrix[(i_var.1, j_var.1)] += 4 * sign * J;

                            // J^a c(i)\sigma^z_i\sigma^z_{j,a}
                            let aux_variable = nb_vars + new_var_counter + j_index;
                            q_matrix[(i_var.1, aux_variable)] += 4 * J * (if i_var.0 { 1 } else { -1 });
                        }
    
                        // h c(i)\sigma^z_i
                        q_matrix[(i_var.1, i_var.1)] = 2 * H * (if i_var.0 { 1 } else { -1 });

                        // h^a_i c(i)\sigma^z_{i,a}
                        let aux_variable = nb_vars + new_var_counter + i_index;
                        let q_i = if i_index == 0 { G / 2 } else { 0 };
                        let h_a_i = - J * (2 * i_index as i32 - clause.len() as i32) + q_i;
                        q_matrix[(aux_variable, aux_variable)] = 2 * h_a_i;
                    }
                            
                    new_var_counter += clause.len();
                }

                for i in 0..(nb_vars+num_ancillae) {
                    let coupling_sum: i32 =(0..(nb_vars+num_ancillae))
                        .filter(|j| i != *j)
                        .map(|j| q_matrix[(i,j)])
                        .sum();
                    q_matrix[(i,i)] -= 2 * coupling_sum;
                }
                assert_eq!(num_ancillae, new_var_counter);

                debug!("Chancellor reduction produced: {}", q_matrix);

                (q_matrix, Box::new(KSatToQuboSolutionReductionReverser::Chancellor(nb_vars)))
            },
            KSatToQuboReduction::Nuesslein2022 => todo!(),
            KSatToQuboReduction::Nuesslein2023 => todo!(),
        }
    }
}

impl SolutionReductionReverser<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboSolutionReductionReverser {
    fn reverse_reduce_solution(&self, solution: QuboSolution) -> SatSolution {
        match self {
            KSatToQuboSolutionReductionReverser::Choi {qubo_reverser, threesat_reverser} => {
                let threesat_solution = qubo_reverser.reverse_reduce_solution(solution);

                threesat_reverser.reverse_reduce_solution(threesat_solution)
            },
            KSatToQuboSolutionReductionReverser::Novel => todo!(),
            KSatToQuboSolutionReductionReverser::Chancellor(nb_vars) => {
                SatSolution::Sat(solution.0[0..*nb_vars].to_vec())
            },
            KSatToQuboSolutionReductionReverser::Nuesslein2022 => todo!(),
            KSatToQuboSolutionReductionReverser::Nuesslein2023 => todo!(),
        }
    }
}
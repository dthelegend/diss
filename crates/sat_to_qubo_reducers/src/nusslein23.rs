use log::{trace, warn};
use nalgebra::{DMatrix, dvector, DVector};
use nalgebra_sparse::CooMatrix;
use common::Reduction;
use qubo_problem::{QuboProblem, QuboSolution, QuboType};
use sat_problem::{KSatProblem, SatSolution, SatVariable};

fn fast_mod2(x: usize) -> usize {
    x & 0x1
}

pub struct Nusslein23 {
    pub nb_vars: usize,
}

impl Reduction<KSatProblem, QuboProblem> for Nusslein23 {
    fn reduce(
        &KSatProblem {
            nb_vars,
            ref clause_list,
        }: &KSatProblem,
    ) -> (QuboProblem, Self) {
        let m = clause_list.len();
        let problem_size = nb_vars + m;
        let mut q_matrix: CooMatrix<QuboType> = CooMatrix::new(problem_size, problem_size);

        for (i, clause) in clause_list.iter().enumerate() {
            let var_clause = nb_vars + i;
            match clause[..] {
                [
                    SatVariable(true, var_a),
                    SatVariable(true, var_b),
                    SatVariable(true, var_c)
                ] => {
                    q_matrix.push(var_a, var_b, 2);
                    q_matrix.push(var_c, var_c, -1);

                    q_matrix.push(var_clause, var_a, -2);
                    q_matrix.push(var_clause, var_b, -2);
                    q_matrix.push(var_clause, var_c, 1);
                    q_matrix.push(var_clause, var_clause, 1);
                }
                [
                SatVariable(true, var_a),
                SatVariable(true, var_b),
                SatVariable(false, var_c)
                ] | [
                SatVariable(true, var_a),
                SatVariable(false, var_c),
                SatVariable(true, var_b)
                ] | [
                SatVariable(false, var_c),
                SatVariable(true, var_a),
                SatVariable(true, var_b)
                ] => {
                    q_matrix.push(var_a, var_b, 2);
                    q_matrix.push(var_c, var_c, 1);

                    q_matrix.push(var_clause, var_a, -2);
                    q_matrix.push(var_clause, var_b, -2);
                    q_matrix.push(var_clause, var_c, -1);
                    q_matrix.push(var_clause, var_clause, 2);
                }
                [
                SatVariable(true, var_a),
                SatVariable(false, var_b),
                SatVariable(false, var_c)
                ] | [
                SatVariable(false, var_b),
                SatVariable(true, var_a),
                SatVariable(false, var_c)
                ] | [
                SatVariable(false, var_b),
                SatVariable(false, var_c),
                SatVariable(true, var_a)
                ] => {
                    q_matrix.push(var_a, var_a, 2);
                    q_matrix.push(var_a, var_b, -2);
                    q_matrix.push(var_c, var_c, 1);

                    q_matrix.push(var_clause, var_a, -2);
                    q_matrix.push(var_clause, var_b, 2);
                    q_matrix.push(var_clause, var_c, -1);
                }
                [
                SatVariable(false, var_a),
                SatVariable(false, var_b),
                SatVariable(false, var_c)
                ] => {
                    q_matrix.push(var_a, var_a, -1);
                    q_matrix.push(var_a, var_b, 1);
                    q_matrix.push(var_a, var_c, 1);
                    q_matrix.push(var_b, var_b, -1);
                    q_matrix.push(var_b, var_c, 1);
                    q_matrix.push(var_c, var_c, -1);

                    q_matrix.push(var_clause, var_a, 1);
                    q_matrix.push(var_clause, var_b, 1);
                    q_matrix.push(var_clause, var_c, 1);
                    q_matrix.push(var_clause, var_clause, -1);
                }
                _ => unimplemented!()
            }
        }

        let problem = QuboProblem::try_from_coo_matrix(&q_matrix)
            .expect("Matrix should be properly formed");

        (problem, Self {
            nb_vars
        })
    }

    fn up_model(&self, QuboSolution(solution_vector): QuboSolution) -> SatSolution {
        let out_sv = DVector::from_fn(self.nb_vars, |i,_| {
            solution_vector[2 * i] != 0
        });

        SatSolution::Sat(out_sv)
    }
}

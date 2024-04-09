use super::{QuboProblem, QuboSolution, QuboType};
use nalgebra::{dvector, DMatrix, DVector};
use nalgebra_sparse::{CooMatrix, CsrMatrix};
use rand::{thread_rng, Rng};

#[test]
fn check_evaluation() {
    let sut_internal = {
        let mut coo_initializer = CooMatrix::new(3, 3);

        coo_initializer.push(0, 0, 1);
        coo_initializer.push(0, 1, 1);
        coo_initializer.push(0, 2, 1);
        coo_initializer.push(1, 1, 1);
        coo_initializer.push(1, 2, 1);
        coo_initializer.push(2, 2, 1);

        CsrMatrix::from(&coo_initializer)
    };

    let sut = QuboProblem::try_from_q_matrix(sut_internal).expect("Matrix is supposedly valid");
    let sut_solution = QuboSolution(dvector![1, 1, 1]);

    assert_eq!(9, sut.evaluate(&sut_solution));
}

#[test]
fn check_delta_evaluation_jk() {
    const PROBLEM_SIZE: usize = 100;

    let mut rng = thread_rng();

    let sut_internal = CsrMatrix::from(&DMatrix::from_fn(PROBLEM_SIZE, PROBLEM_SIZE, |i, j| {
        if i <= j {
            rng.gen_range(0..128)
        } else {
            0
        }
    }))
    .upper_triangle();

    let sut = QuboProblem::try_from_q_matrix(sut_internal).expect("Matrix is supposedly valid");
    let sut_solution = QuboSolution(DVector::from_iterator(
        PROBLEM_SIZE,
        (0..PROBLEM_SIZE).map(|_| rng.gen_range(0..=1)),
    ));

    let eval = sut.evaluate(&sut_solution);

    for j in 0..PROBLEM_SIZE {
        for k in 0..PROBLEM_SIZE {
            // Delta(k, X)
            let sut_solution_k = sut_solution.flip(k);
            let eval_k = sut.evaluate(&sut_solution_k);
            let delta_k = eval_k - eval;

            // D(k, f(j, X)) = E(f(k, f(j, X)) - E(f(j, X))
            let delta_eval_k_and_eval_j =
                //                              X              D(k, X)  j  k
                sut.flip_j_and_delta_evaluate_k(&sut_solution, delta_k, j, k);

            let delta_kj_k =
                sut.evaluate(&sut_solution_k.flip(j)) - sut.evaluate(&sut_solution.flip(j));

            assert_eq!(delta_kj_k, delta_eval_k_and_eval_j);
        }
    }
}

#[test]
fn check_delta_evaluation_k() {
    const PROBLEM_SIZE: usize = 100;

    let mut rng = thread_rng();

    let sut_internal = CsrMatrix::from(&DMatrix::from_fn(PROBLEM_SIZE, PROBLEM_SIZE, |i, j| {
        if i <= j {
            rng.gen_range(0..128)
        } else {
            0
        }
    }))
    .upper_triangle();

    let sut = QuboProblem::try_from_q_matrix(sut_internal).expect("Matrix is supposedly valid");

    println!("Generated problem: {:?}", sut);

    let sut_solution = QuboSolution(DVector::from_fn(PROBLEM_SIZE, |_, _| rng.gen_range(0..=1)));

    let eval = sut.evaluate(&sut_solution);

    for k in 0..PROBLEM_SIZE {
        let sut_solution_k = sut_solution.flip(k);

        let eval_k = sut.evaluate(&sut_solution_k);

        let delta_k = sut.delta_evaluate_k(&sut_solution, k);

        assert_eq!(eval_k - eval, delta_k);

        let delta_k_neg = sut.delta_evaluate_k(&sut_solution_k, k);

        assert_eq!(0, delta_k_neg + delta_k);
    }
}

#[test]
fn check_ising() {
    const PROBLEM_SIZE: usize = 100;
    
    let mut rng = thread_rng();
    
    let sut_internal: CsrMatrix<QuboType> = CsrMatrix::from(&DMatrix::from_fn(PROBLEM_SIZE, PROBLEM_SIZE, |i, j| {
        if i <= j {
            rng.gen_range(-128..128)
        } else {
            0
        }
    }));
    
    let h : Vec<_> = sut_internal.diagonal_as_csr().triplet_iter().map(|(i,_,v)| (i,*v)).collect();
    let j : Vec<_> = sut_internal.triplet_iter().filter_map(|(i,j,v)|(i != j).then_some((i, j, *v))).collect();
    
    let (yy, _) = QuboProblem::try_from_ising_triplets(PROBLEM_SIZE, j.clone(), h.clone()).unwrap();

    let (h_t, mut j_t) = yy.get_ising();

    j_t.set_diagonal(&h_t);
    assert_eq!(sut_internal * DMatrix::identity(PROBLEM_SIZE, PROBLEM_SIZE), j_t / 4);
}

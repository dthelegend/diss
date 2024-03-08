use super::{QuboProblem, QuboSolution};
use nalgebra::DVector;
use nalgebra_sparse::{CooMatrix, CsrMatrix};

#[test]
fn check_evaluation() {
    let sut_internal = {
        let mut coo_initializer = CooMatrix::new(3, 3);

        coo_initializer.push(0, 0, 1);
        coo_initializer.push(0, 1, 1);
        coo_initializer.push(0, 2, 1);
        coo_initializer.push(1, 1, 2);
        coo_initializer.push(1, 2, 1);
        coo_initializer.push(2, 2, 1);

        CsrMatrix::from(&coo_initializer)
    };

    let sut =
        QuboProblem::try_from_q_matrix(sut_internal.into()).expect("Matrix is supposedly valid");
    let sut_solution = QuboSolution(DVector::from_column_slice(&[1, 1, 1]));

    assert_eq!(7, sut.evaluate(&sut_solution));
}

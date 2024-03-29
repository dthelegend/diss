
use rand::prelude::ThreadRng;
use rand::thread_rng;
use common::Solver;
use qubo_problem::{QuboProblem, QuboSolution};
use crate::mopso::mopso_gpu::gpu_mopso_helper;

mod mopso_gpu {
    use nalgebra::DMatrix;
    use rand::Rng;
    use qubo_problem::{QuboProblem, QuboType};

    #[link(name = "kernels")]
    extern "C" {
        fn run_mopso_solver(
            problem_size: usize,
            qubo_problem: *const QuboType,
            solutions_flat: *const QuboType,
            number_of_particles: usize
        );
    }

    pub fn gpu_mopso_helper(
        mut rng: impl Rng,
        qubo_problem: &QuboProblem,
        number_of_particles: usize,
    ) {
        let solutions_flat: DMatrix<QuboType> =
            DMatrix::from_fn(number_of_particles, qubo_problem.get_size(), |_, _| {
                rng.gen_range(0..=1)
            });

        let dense_problem = qubo_problem.get_dense();

        unsafe {
            run_mopso_solver(
                qubo_problem.get_size(),
                dense_problem.as_ptr(),
                solutions_flat.as_ptr(),
                number_of_particles,
            )
        }

        todo!()
    }
}

pub struct Mopso<Rng>
where
    Rng: rand::Rng,
{
    rng: Rng,
    number_of_particles: usize,
    max_iterations: usize,
}

impl<Rng> Mopso<Rng>
where
    Rng: rand::Rng,
{
    pub fn new_with_rng(rng: Rng, number_of_particles: usize, max_iterations: usize) -> Self {
        Self {
            rng,
            number_of_particles,
            max_iterations,
        }
    }
}

impl Mopso<ThreadRng> {
    pub fn new_with_thread_rng(number_of_particles: usize, max_iterations: usize) -> Self {
        Self::new_with_rng(thread_rng(), number_of_particles, max_iterations)
    }
}

impl<Rng> Solver<QuboProblem> for Mopso<Rng>
where
    Rng: rand::Rng,
{
    fn solve(&mut self, qubo_problem: &QuboProblem) -> QuboSolution {
        gpu_mopso_helper(&mut self.rng, qubo_problem, self.number_of_particles);

        todo!()
    }
}

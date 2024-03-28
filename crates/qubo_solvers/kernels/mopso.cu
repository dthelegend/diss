#include <iostream>
#include "qubo_solvers.h"

extern "C" void run_mopso_solver(
            size_t problem_size,
            const qubo_t* qubo_problem,
            const qubo_t* solutions_flat,
            size_t number_of_particles) {
    std::cout << "Hello from CUDA! (MOPSO)" << std::endl;
}

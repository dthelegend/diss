#include "qubo_solvers.h"
#include <iostream>

const size_t MAX_DEPTH = 16;
const size_t MAX_PROBLEM_SIZE = MAX_DEPTH;
const size_t MAX_SUB_PROBLEMS = 1 << (MAX_DEPTH - 1);

__device__ size_t n_sub_problems;

__device__ size_t d_problem_size;
__device__ qubo_t d_qubo_problem[MAX_PROBLEM_SIZE][MAX_PROBLEM_SIZE];

__device__ qubo_t d_curr_solution[MAX_PROBLEM_SIZE];
__device__ qubo_t d_curr_deltas[MAX_SUB_PROBLEMS][MAX_PROBLEM_SIZE];
__device__ qubo_t d_curr_eval[MAX_SUB_PROBLEMS];

__device__ size_t d_min_solution_index = 0;
__device__ qubo_t d_min_solution[MAX_PROBLEM_SIZE];
__device__ qubo_t d_min_eval;

__global__ void __launch_bounds__(MAX_PROBLEM_SIZE, MAX_SUB_PROBLEMS) flip_part_one(
    size_t i
) {
    size_t sub_problem_start_index = blockIdx.x * blockDim.x;
    int sub_problem_stride = blockDim.x * gridDim.x;

    size_t j = threadIdx.x;

    for (
        auto sub_problem_index = sub_problem_start_index;
        sub_problem_index < n_sub_problems;
        sub_problem_index += sub_problem_stride
    ) {
        d_curr_deltas[sub_problem_index][i] += 2 * d_qubo_problem[i][j] * (2 * d_curr_solution[i] - 1) * (2 * d_curr_solution[j] - 1);
    }
}

__global__ void __launch_bounds__(MAX_PROBLEM_SIZE, MAX_SUB_PROBLEMS) flip_part_two(
    size_t i
) {
    size_t sub_problem_start_index = blockIdx.x * blockDim.x + threadIdx.x;
    int sub_problem_stride = blockDim.x * gridDim.x;

    if (sub_problem_start_index == 0) {
        d_curr_solution[i] = 1 - d_curr_solution[i];
    }

    for (
        auto sub_problem_index = sub_problem_start_index;
        (sub_problem_index < n_sub_problems) && (threadIdx.x == 0);
        sub_problem_index += sub_problem_stride
    ) {
        d_curr_eval[sub_problem_index] += d_curr_deltas[sub_problem_index][i];
        d_curr_deltas[sub_problem_index][i] = - d_curr_deltas[sub_problem_index][i];
    }
}

__global__ void __launch_bounds__(MAX_PROBLEM_SIZE, MAX_PROBLEM_SIZE) set_best_solution(
    size_t sub_problem_index
) {
//     size_t start_index = blockIdx.x * blockDim.x + threadIdx.x;
//     int stride = blockDim.x * gridDim.x;
//
//     if (start_index == 0) {
//         d_min_solution_index = sub_problem_index;
//     }
//
//     for (size_t index = start_index; index < d_problem_size; index += stride) {
//         d_curr_solution[index] = d_curr_solution[index];
//     }
}

__global__ void
__launch_bounds__(1, 1)
flip(
    size_t i
) {
    if (i > 0) {
        flip_part_one<<<i,n_sub_problems>>>(i);
    }
    // TODO maybe increase thread use by calculating this
    flip_part_two<<<1,n_sub_problems>>>(i);
}

__global__ void
__launch_bounds__(1, 1)
search(
    size_t i
) {
    printf("%d\n", i);
    if (i == 0) {
//         size_t min_eval_index = 0;
//
//         for (size_t j = 1; j < n_sub_problems; j++) {
//             printf("%d", d_curr_eval[j]);
//             if (d_curr_eval[j] < d_curr_eval[min_eval_index]) {
//                 min_eval_index = j;
//             }
//             printf("\n");
//         }
//
//         if (d_curr_eval[min_eval_index] < d_min_eval) {
//             set_best_solution<<<d_problem_size, 1>>>(min_eval_index);
//         }
    } else {
//         search<<<1,1>>>(i - 1);

        flip<<<1,1>>>(i - 1);

//         search<<<1,1>>>(i - 1);

        flip<<<1,1>>>(i - 1);
    }
}

extern "C" cudaError_t run_pes_solver(
    const size_t problem_size,
    const qubo_t* qubo_problem,
    qubo_t* best_solution,
    qubo_t* best_evaluation,
    const qubo_t* solution_list,
    const qubo_t* deltas_list,
    const qubo_t* eval_list,
    const size_t i
) {
    size_t n = 1 << (problem_size - i - 1);

    std::cout << "Hello from CUDA! 0" << std::endl;
    cudaCheckError(cudaMemcpy(&n_sub_problems, &n, sizeof(size_t), cudaMemcpyDefault));
    std::cout << "Hello from CUDA! 1" << std::endl;

    cudaCheckError(cudaMemcpy(&d_problem_size, &problem_size, sizeof(size_t), cudaMemcpyDefault));
    cudaCheckError(cudaMemcpy2D(d_qubo_problem, MAX_PROBLEM_SIZE * sizeof(qubo_t), qubo_problem, problem_size * sizeof(qubo_t), i * sizeof(qubo_t), i, cudaMemcpyDefault));

    std::cout << "Hello from CUDA! 2" << std::endl;
    cudaCheckError(cudaMemcpy2D(d_curr_solution, MAX_PROBLEM_SIZE * sizeof(qubo_t), solution_list, problem_size * sizeof(qubo_t), i * sizeof(qubo_t), n, cudaMemcpyDefault));
    cudaCheckError(cudaMemcpy2D(d_curr_deltas, MAX_PROBLEM_SIZE * sizeof(qubo_t), deltas_list, problem_size * sizeof(qubo_t), i * sizeof(qubo_t), n, cudaMemcpyDefault));
    cudaCheckError(cudaMemcpy(d_curr_eval, eval_list, sizeof(qubo_t) * n, cudaMemcpyDefault));

    std::cout << "Hello from CUDA! 3" << std::endl;
    cudaCheckError(cudaMemcpy(d_min_solution, best_solution, sizeof(qubo_t) * problem_size, cudaMemcpyDefault));
    cudaCheckError(cudaMemcpy(&d_min_eval, best_evaluation, sizeof(qubo_t), cudaMemcpyDefault));

    std::cout << "Hello from CUDA! 4" << std::endl;

    search<<<1,1>>>(i);

    std::cout << "Hello from CUDA! 5" << std::endl;

    cudaCheckError(cudaGetLastError());

    std::cout << "Hello from CUDA! 6" << std::endl;

    cudaDeviceSynchronize();

    std::cout << "Goodbye from CUDA! 7" << std::endl;

    cudaCheckError(cudaMemcpy(best_solution, d_min_solution, sizeof(qubo_t) * problem_size, cudaMemcpyDefault));
    cudaCheckError(cudaMemcpy(best_evaluation, &d_min_eval, sizeof(qubo_t), cudaMemcpyDefault));

    return cudaSuccess;
}

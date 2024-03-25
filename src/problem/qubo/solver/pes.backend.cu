#include <iostream>

typedef int32_t qubo_t;

__device__
void flip_each(
    const size_t problem_size,
    const qubo_t* qubo_problem,
    qubo_t* deltas,
    qubo_t* solution_list,
    qubo_t* eval,
    const size_t i
) {
    *eval += deltas[i];
    for (size_t j = 0; j < i; j++) {
        deltas[j] += 2 * qubo_problem[j * problem_size + i] * (2 * solution_list[i] - 1) * (2 * solution_list[j] - 1);
    }
    deltas[i] = - deltas[i];
    solution_list[i] = 1 - solution_list[i];
}

__global__
void flip(
    const size_t n,
    const size_t problem_size,
    const qubo_t* qubo_problem,
    qubo_t* deltas,
    qubo_t* solution_list,
    qubo_t* eval_list,
    const size_t alpha
) {
    int index = blockIdx.x * blockDim.x + threadIdx.x;
    int stride = blockDim.x * gridDim.x;
    for (int i = index; i < n; i += stride) {
        flip_each(
            problem_size,
            qubo_problem,
            deltas + (i * problem_size),
            solution_list + (i * problem_size),
            eval_list + i,
            alpha
        );
    }
}

__host__ void search(
    const size_t n,
    const int num_blocks,
    const int block_size,
    const size_t problem_size,
    const qubo_t* qubo_problem,
    qubo_t* best_solution,
    qubo_t* best_evaluation,
    qubo_t* deltas,
    qubo_t* solution_list,
    qubo_t* eval_list,
    const size_t i
) {
    if (i == 0) {
        // Check all values for minimum
        // This could in theory be parallelised to log n, but the constant factor tends to be relatively small
        size_t min_eval_index = 0;

        for (size_t j = 1; j < n; j++) {
            if (eval_list[j] < eval_list[min_eval_index]) {
                min_eval_index = j;
            }
        }

        if (eval_list[min_eval_index] < *best_evaluation) {
            cudaMemcpy(best_evaluation, eval_list + min_eval_index, sizeof(qubo_t), cudaMemcpyDeviceToHost);
            cudaMemcpy(best_solution, solution_list + (min_eval_index * problem_size), problem_size * sizeof(qubo_t), cudaMemcpyDeviceToHost);
        }

        return;
    }
    
    // search left
    search(
        n,
        num_blocks,
        block_size,
        problem_size,
        qubo_problem,
        best_solution,
        best_evaluation,
        deltas,
        solution_list,
        eval_list,
        i - 1
    );

    flip<<<num_blocks, block_size>>>(
        n,
        problem_size,
        qubo_problem,
        deltas,
        solution_list,
        eval_list,
        i - 1
    );

    cudaDeviceSynchronize();

    search(
        n,
        num_blocks,
        block_size,
        problem_size,
        qubo_problem,
        best_solution,
        best_evaluation,
        deltas,
        solution_list,
        eval_list,
        i - 1
    );

    flip<<<num_blocks, block_size>>>(
        n,
        problem_size,
        qubo_problem,
        deltas,
        solution_list,
        eval_list,
        i - 1
    );

    cudaDeviceSynchronize();
}

extern "C" void run_pes_solver(
    const int block_size,
    const size_t problem_size,
    const qubo_t* qubo_problem,
    qubo_t* best_solution,
    qubo_t* best_evaluation,
    const qubo_t* deltas,
    const qubo_t* solution_list,
    const qubo_t* eval_list,
    const size_t i
) {
    std::cout << "Hello from CUDA!" << std::endl;

    int n = 1 << (problem_size - i - 1);
    int num_blocks = (n + block_size - 1) / block_size;
    
    // TODO Throw an exception if the memory cannot b alloc'd
    
    qubo_t* cuda_qubo_problem;
    cudaMallocManaged(&cuda_qubo_problem, problem_size * problem_size * sizeof(qubo_t));
    cudaMemcpy(cuda_qubo_problem, qubo_problem, problem_size * problem_size * sizeof(qubo_t), cudaMemcpyHostToDevice);

    qubo_t* cuda_deltas;
    cudaMallocManaged(&cuda_deltas, n * problem_size * sizeof(qubo_t));
    cudaMemcpy(cuda_deltas, deltas, n * problem_size * sizeof(qubo_t), cudaMemcpyHostToDevice);

    qubo_t* cuda_solution_list;
    cudaMallocManaged(&cuda_solution_list, n * problem_size * sizeof(qubo_t));
    cudaMemcpy(cuda_solution_list, solution_list, n * problem_size * sizeof(qubo_t), cudaMemcpyHostToDevice);

    qubo_t* cuda_eval_list;
    cudaMallocManaged(&cuda_eval_list, n * sizeof(qubo_t));
    cudaMemcpy(cuda_eval_list, eval_list, n * sizeof(qubo_t), cudaMemcpyHostToDevice);

    search(
        n,
        num_blocks,
        block_size,
        problem_size,
        cuda_qubo_problem,
        best_solution,
        best_evaluation,
        cuda_deltas,
        cuda_solution_list,
        cuda_eval_list,
        i
    );

    std::cout << "Goodbye from CUDA!" << std::endl;
}
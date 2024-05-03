#ifndef QUBO_SOLVERS_H
#define QUBO_SOLVERS_H

typedef int32_t qubo_t;

#define cudaCheckError(v) \
    { \
        { \
            cudaError_t error = v; \
            if (error != cudaSuccess) { \
                return error; \
            } \
        } \
    }

#endif
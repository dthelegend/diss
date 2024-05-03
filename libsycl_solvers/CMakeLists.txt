cmake_minimum_required(VERSION 3.29)

set(CMAKE_CXX_COMPILER "clang++")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsycl -fsycl-link")

project(sycl_solvers LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

add_library(sycl_solvers STATIC
        momentum.cpp
        mopso.cpp)

set_property(TARGET sycl_solvers PROPERTY POSITION_INDEPENDENT_CODE ON)

target_include_directories(sycl_solvers PRIVATE include)

install(TARGETS sycl_solvers
    ARCHIVE DESTINATION .
)

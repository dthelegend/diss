#import "@preview/charged-ieee:0.1.0": ieee

#let title = "QuboSAT: Leveraging QUBO for Parallel and Quantum Boolean Satisfiability"

#show: ieee.with(
    title: title,
    subtitle: "Analysis of reductions from SAT to QUBO and an implementation of a solver for the satisfiability problem using these techniques.",
    authors: (
        (
        name: "Daudi Wampamba",
        affiliation: "University of Nottingham",
        email: "psydw3@nottingham.ac.uk",
        ),
    ),
    index-terms: (
        "Quadratic Unconstrained Binary Optimization",
        "Satisfiability",
        "Solvers"
    ),
    review: false,
    bibliography-file: "main.bib",
    abstract: [Solving the boolean satisfiability (SAT) problem is a fundamental challenge in computer science with wide-ranging applications. This project investigates using reductions from SAT to the quadratic unconstrained binary optimization (QUBO) problem as a novel approach to SAT solving. By leveraging substantial prior work on SAT-to-QUBO reductions and solving QUBO problems with quantum and parallel computing techniques, we aim to create a high-performance parallel SAT solver. Several existing SAT-to-QUBO reduction algorithms are implemented, including approaches by Choi, Chancellor, and Nusslein. The reduced QUBO instances are then solved using techniques like simulated annealing, parallel exhaustive search, and momentum annealing. Benchmarking is performed on standard SAT Competition instances, comparing solving time and reduction quality metrics like fitness distance correlation. While some features like GPU acceleration were left unimplemented, the flexible design allows straightforward extension to new reductions and solvers. Overall, this work provides a toolkit for developing QUBO-based optimization problem solvers with potential applications in quantum computing.]
)

= Introduction

In the realm of computational complexity, the Boolean Satisfiability Problem (SAT) stands as a fundamental challenge in modern computing. One of the first problems to be shown as NP-complete, there exists a wide body of work that has sought build and improve our approach to solving SAT problems with implications across various fields, ranging from artificial intelligence, to hardware verification and design, to scheduling, and more. As the size and complexity of SAT problems grow however, traditional methods begin to show their weakness.

This project focuses on studying the gap between SAT and QUBO to create a SAT solver that employs a reduction from SAT to QUBO as a means to study the characteristics of such reductions and provide a platform that could potentially be expanded to run in on quantum computers. Leveraging the substantial body of research surrounding the reduction of SAT problems to QUBO and implementing QUBO on quantum and parallel computers, I aim to not only to create a parallel SAT solver, but also compare this approach with existing approaches.

== Parallel & Quantum Computing

In order to search for ways to increase performance, we can look to the other approaches that have shown promise in other NP-Hard and NP-Complete problems.

Parallel computing has become a pivotal technology in the modern world, playing an increasing role in recent years as the main method of improving the performance of classical algorithms. The increasing complexity of computational problems, along with the increasing prevalence of powerful parallel processors and accelerator hardware, has pushed industry and research to look to the efficient parallelisation of work as a solution to dealing with ever larger problems.

While parallel computing has gained prominence as a reliable method for improving performance, quantum computing remains in its experimental infancy, characterized by the unique abilities that quantum effects enable that offer the potential for groundbreaking speed-ups. These two computing paradigms have given rise to divergent strategies in algorithm development and problem-solving approaches.

= Background

== The SAT Problem and Modern Parallel SAT Solvers

The SAT problem poses that given a set of boolean variables $S = {s_1, s_2, ...,  s_n}$ and a set of clauses that link these variables $C = {c_1, c_2, ..., c_n}$, find the configuration of the boolean variables $S^*$ that satisfies all clauses (if it exists).

Modern sequential SAT Solvers rely on an iterative process of Boolean Constraint Propogation (BCP). BCP is a P-Hard Problem, and as such is naturally hard to parallelise. To get around this, modern parallel SAT Solvers use a mix of portfolio solving (where multiple solvers search the same space, but along different paths; This process can either be deterministic or stochastic), search space splitting, and load balancing in order to acheive parallelisation @martins_overview_2012. These methods are good enough that they will on average perform better than an equivalent sequential solvers but they suffer from poor overall core-utilisation, and memory access limitations when sharing clauses between threads.

== The Quadratic Unconstrained Binary Optimization Problem

The QUBO problem asks that given the boolean vector $x = { x_1, x_2, ..., x_n }$ and a symmetric/upper-triangular matrix of unconstrained values $Q in RR^(n,n)$ find the configuration of boolean variables $x^*$ that minimises the equation $x^T Q x$.

One notable feature of QUBO problems is their suitability for quantum computers, specifically through a technique known as quantum annealing. Quantum annealing is a quantum computing approach designed to efficiently find the ground state of an ising model. Ising models are normally defined by a hamiltonian function $E = sum h_i sigma_i + sum J_(<i, j>) sigma_i sigma_j$ which defines a set of biases $h$ and couplings $J$ between spin variables $sigma in { -1, 1 }$. By leveraging the principles of quantum mechanics to encode the QUBO solution in an ising model, quantum annealing can potentially solve QUBO faster than classical computers for certain optimization tasks and with significantly better scaling to larger problems.

Due to the equivalence of the QUBO and ising model, QUBO has garnered significant research interest. This interest has also lead to an interest in solving QUBO problems using parallel computing, not only quantum computing. Various parallelization techniques have been developed for QUBO problems. Some of these techniques aim to simulate the quantum annealing process using parallel computing hardware, while others explore novel approaches to solve QUBO problems.

= Related Work

This project builds off a myriad of work in SAT reductions and in the field of Parallel QUBO.

== Reductions

The SAT to QUBO reductions implemented for this paper are:

=== *Choi* @choi_different_2011
This algorithm uses a reduction from K-SAT to 3-SAT to MIS then finally QUBO. The implementation I used for this paper converts choi directly from k-sat to QUBO. By connecting every node in a clause together we can produce a sub-graph for a clause. We then generate a sub-graph for every node and then with the sub-graph we connect any nodes with any conflicts (e.g. $not x$ and $x$). These graphs are equivalent to the corresponding QUBO problem in the adjacency matrix format.

=== *Chancellor* @chancellor_direct_2016
This directly encodes problems in a Hamiltonian function for an ising model. That can be converted into a QUBO problem for our solver in $O(n^2)$ time. Each clause has an energy that is either $0$ if it is satisfied, or some positive value $g$ if it is not satisfied. This energy is summed across all the clauses to create the overall hamiltonian. This is the current state-of-the-art method, and the resulting QUBO Matrices are notably smaller than that of Choi for large problems.

=== *Nusslein 2022* @nuslein_algorithmic_2022
Scales better than chancellor for K-SAT problems by using a combination of efficient Ising reductions for clauses of length $2$ and $4+$. It scales better for QUBO formulations where the resulting QUBO graph has a number of edges that is sub-quadratic i.e. $|E| = Theta(|V|)$

=== *Nusslein 2023* @nuslein_solving_2023
There are two formulations listed in this paper. Both are implemented, however, we will focus on the $(n+m)$ reduction.
It produces sane sized QUBO matrices to Chancellor with similar characteristics, however, it is restricted to 3-SAT. This reduction is also multi-objective, requiring the maximisation of $|sum x_i|$ in order for it to produce SAT outcomes.

== Solvers

=== *Simulated Annealing*
This is implemented as a reference algorithm to show how a simple optimisation algorithm performs across the different reductions

=== *Parallel Exhaustive Search* @tao_work-time_2020
This algorithm is implemented both sequentially and in parallel and provides a Work-time optimal way to search the entire QUBO space. The complexity of this is $O(2^n / p +n^2)$.

=== *Momentum Annealing* @okuyama_binary_2019
Momentum annealing is similar to simulated annealing as a markov chain process, but unlike simulated annealing, momentum annealing uses a second-order markov chain that uses a bipartite representation of the ising model that then gradually stabilises both sides of the graph to the lowest energy.

= Design

The requirements for the solver are laid out below:
- It must be flexible and allow for the implementation of many alternative backends and reductions
- It must be simple to use, whether as a standalone solver, or as a library for integration into other work
- It must be extensible by third parties with custom out-of-repository reduction and solving algorithms
- It must support the widely recognised standards for input and output

The general flow for solving a problem is shown in @solver-layout. Input from the user is processed into the SAT Problem instance. The SAT Problem is then reduced into a QUBO problem using one of the specified algorithms. We can then solve the reduced QUBO problem with another specified algorithm. Once we have a solution, we present it to the user.

#figure(image("./solver_layout.png"), caption: [The data flow from input to output of the solver], placement: auto) <solver-layout>

The fundamental design of the logger has stayed mostly the same, over the course of the project, as the flow is relatively lean. There is additionally a logger that can be used to record information about the energy of the solution as the solver progresses.

= Implementation

The Solver is written in Rust. Rust was chosen as it has good tools for abstraction of problems and a strong type system that makes it easy to encode problems in. Additionally it is fast and has great tools for CPU concurrency which made coding the parallel sections much easier.

== Input/Output

Following typical SAT solver convention and in-line with the competition requirements, the solver uses the DIMACS Input format for inputing problem instances. Included in this project is a python file `generate_cnf.py` for generating random cnf instances.

The solver then outputs `SAT` with a model, `UNSAT`, or `UNKNOWN`.

== Problem & Solution Encoding

Problems are implemented as Rust traits which makes it easy to implement different problems which can be reduced into one another in definitive ways. There is an interface that all problems implement, `Problem`. There is also `Reduction` trait that provides an interface for problems to be reduced from one to another. Using a reduction to solve a problem creates a new struct with the information needed for up-modelling solutions without requiring the whole original problem. The solver interface allows the easy implementation of solvers for problems.

```rust
pub trait Problem {
    type Solution;
}

pub trait Reduction<U, V>
where
    U: Problem,
    V: Problem,
{
    fn reduce(problem: &U) -> (V, Self);

    fn up_model(&self, solution: V::Solution) -> U::Solution;
}

pub trait Solver<T>
where
    T: Problem,
{
    fn solve(&mut self, problem: &T, logger: Option<impl DataRecorder>) -> T::Solution;
}
```

This makes it trivial to add new Problems to the solver as intermediary reductions for example the Choi KSAT to QUBO reduction is shown.

```rust
// Choi scales directly in the number of clause variables and therefore the size of the problem
pub struct Choi {
    map: Vec<(Vec<usize>, Vec<usize>)>,
}

impl Reduction<KSatProblem, QuboProblem> for Choi {
    fn reduce(sat_problem: &KSatProblem) -> (QuboProblem, Self) {
        ...
    }

    fn up_model(&self, qubo_solution: QuboSolution) -> SatSolution {
        ...
    }
}
```

The underlying structure of problems can vary a lot, but rust allows us to be flexible with that as long as we implement the traits above. The QUBO problem uses a sparse matrix representation in order to improve memory efficiency. The problem representation is agnostic to the reduction used and the solver used to generate its solutions.

```rust
// ...
pub struct QuboProblem {
    problem_matrix: matrix::SparseMatrix<i32>,
    ...
}
```

The SAT problem doesn't have any implemented solvers as it is tangential to this project. SAT instances are stored as the number of variables as a list of of lists of variables analagous to conjunctive normal form, each variable having whether it is negated (`true` for not negated, `false` for negated) and a which variable number it corresponds to. The alternative form is storing a large binary matrix with $2 * N$ columns and $M$ rows. This style of matrix has faster row/column access and is faster for calculating satisfiability, but is slower to access the list indexes of variables in clauses (this operation is $O(n)$), which is trivial in the representation used (the same operation is $O(1)$), and is one of the primary operations for reductions.

```rust
pub struct KSatVariable(pub bool, pub usize);

pub struct KSatProblem(pub usize, Vec<Vec<KSatVariable>>);
```

Solutions can be any rust type which makes defining problems very flexible, for example when solving a SAT problem there are 3 possible outcomes, `SAT` (with a model) or `UNSAT` or `UNKNOWN`, however for QUBO problems, there is only one form of solution which is the list of binary variables.

```rust
pub enum KSatSolution {
    Sat(Vec<bool>),
    Unsat,
    Unknown
}
// ...
pub struct QuboSolution(pub DVector<bool>);
```

The problem, reduction, and solver traits have changed significantly over the course of the project. Initially there was an overly complex nest of trais and implementations, sub-implementations, and state passing. This led to a frustration with implementing new solvers and reductions, and the eventual removal of these traits and a reversion to simply using the structs as is. Eventually with more familiarity with the rust programming language, I was able to slim down the traits such that they could be easily reimplemented, and still provide the same level of flexibility that I was trying to acheive at the beginning.

The package (referred to as crates in rust) stucture has also seen a massive overhaul to support seriously extending the project into something that is not only maintainable, but also can accomodate external collaborators and expansion. The project is now split into 6 separate crates, each problem has a crate dedicated to it, while reductions and solvers share their respective crates. The project has a common crate which then provides the common interfaces and finally a CLI crate that contains a binary that an end-user can run the solver on `.cnf` files.

= Evaluation

== Methodology

The solver will be evaluted using the #link("https://satcompetition.github.io/2023/downloads.html")[SAT Competition 2023 Parallel Track benchmarks]. This provides a set of comprehensive benchmarks over both real and theoretical problems, and a standard point of comparison with a swathe of modern parallel SAT solvers. The main metric that will be measured for the solvers is time to solve. The time for the reduction is not taken into account as the reduction is by far the fastest operation in the solver pipeline and as polynomial time operations, they are not major contributors to the overall running time of the program.

The main way that we will analyse our reductions is using fitness distance correlation coefficient@fdcc with the hamming distance from the global optimum (or the first global optimum with the most true variables). This is a common benchmark for search landscape analysis for evaluating fitness functions, but serves an identical role here with the fitness function being the evaluation function of the QUBO matrix. This allows for the objective measurement of how well each of the reductions performed with our solvers. The growth characteristics of each of the reduction's are already well understood and can be read in more depth in their respective papers.

== Results

DATA MISSING

= Reflections

== Timeframes and Goals

TODO

== Difficulties Encountered & Unimplemented Features

=== GPU programming & GPU Parallel MOPSO

GPU programming is still in its infancy in Rust, however, I managed to develop an initial solution with C FFI to bind some CUDA code to my code. The CUDA code however did not perform nearly as well as the paper I was reprodicing with similar hardware. After failing to get more information on the GPU code, I contacted the paper authors, however they were not able to provide the source for my use.

The current code has a solution for binding GPU C++ code using SYCL, however there was not enough time after implementing that for GPU Parallel MOPSO, thusly it was not implemented/evaluated for this paper.

=== CLI

The command line interface of the program is sufficient for testing and collecting data, but there are some smaller convenience features that were never integrated, namely Backend Selection, Reduction Selection, and convenience methods for verifying problems.

=== Logging

Although logging is supported, I am not very happy with the implementation of logging as it is, it could be better with use of the native logging combined with structured logging rather than using a CSV writer.

== Future development

The project is adequately laid out for supporting future development and especially 3rd party libraries and integrations for adding new problems, reductions, and solvers, and integrating it into modern applications. In future I would like to better document the APIs that are exposed and produce a more expansive set of solvers. Due to the underlying flexibility if tge code, I can see this becoming the basis of a new library of reductions and solvers for the Rust programming language.

Potentially, using PyO3 for Rust, this project could be extended to provide APIs for python, providing a fast solver platform for a wider audience of researchers and students to implement their own solvers. Providing a toolkit to build hard optimisation problem solvers and reducers is also what I believe is the largest contribution of this project. The code is released under the LGPL license and available on #link("github", "https://github.com/dthelegend/diss") and will be renamed "optimise-rs".

= Summary

Key potential future directions are better documenting the exposed APIs, implementing a more expansive set of solver algorithms, providing Python bindings via PyO3, and positioning QuboSAT as a generalized toolkit for developing solvers for hard optimization problems across domains, including quantum computing applications that could leverage the QUBO reductions.

Overall, this work demonstrated the feasibility of using QUBO reductions for SAT solving and provided a flexible codebase that can serve as a foundation for further research into high-performance optimization solvers leveraging both classical parallel computing and quantum computing techniques.

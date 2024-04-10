#import "acmart.typ": acmart

#let title = "QuboSAT: A Boolean Satisfiability solver using a QUBO Reduction"

#show: body => acmart(
    title: title,
    subtitle: "Analysis of reductions from SAT to QUBO and am implementation of the solver for the satisfiability problem using a reduction to the",
    authors: (
        (
        name: "Daudi Wampamba",
        affiliation: "University of Nottingham",
        email: "psydw3@nottingham.ac.uk",
        ),
    ),
    ccs-concepts: (),
    keywords: (
        "Quadratic Unconstrained Binary Optimization",
        "Satisfiability",
        "Solvers"
    ),
    conference: (
        name: "University of Nottingham Master's Dissertation",
        short: "Master's Dissertation",
        year: "2024",
        date: "April",
        venue: "Nottingham, UK",
    ),
    copyright: (
        doi: "N/A",
        isbn: "N/A",
        price: "Free",
        mode: "rightsretained",
    ),
    review: false,
    bibliography-file: "main.bib",
    body
)

= Introduction

In the realm of computational complexity, the Boolean Satisfiability Problem (SAT) stands as a fundamental challenge in modern computing. One of the first problems to be shown as NP-complete, there exists a wide body of work that has sought build and improve our approach to solving SAT problems with implications across various fields, ranging from artificial intelligence, to hardware verification and design, to scheduling, and more. As the size and complexity of SAT problems grow however, traditional methods begin to show their weakness.

== Parallel & Quantum Computing

In order to search for ways to increase performance, we can look to the other approaches that have shown promise in other NP-Hard and NP-Complete problems.

Parallel computing has become a pivotal technology in the modern world, playing an increasing role in recent years as the main method of improving the performance of classical algorithms. The increasing complexity of computational problems, along with the increasing prevalence of powerful parallel processors and accelerator hardware, has pushed industry and research to look to the efficient parallelisation of work as a solution to dealing with ever larger problems.

While parallel computing has gained prominence as a reliable method for improving performance, quantum computing remains in its experimental infancy, characterized by the unique abilities that quantum effects enable that offer the potential for groundbreaking speed-ups. These two computing paradigms have given rise to divergent strategies in algorithm development and problem-solving approaches.

== The SAT Problem and Modern Parallel SAT Solvers

TODO: Define SAT with math
// Given a set of boolean variables $S = {s_1, s_2, ...,  s_n}$ and a set of clauses $C = {c_1, c_2, ..., c_n}$

// $ display(and_x^2 or_) $

Modern sequential SAT Solvers rely on an iterative process of Boolean Constraint Propogation (BCP). BCP is a P-Hard Problem, and as such is naturally hard to parallelise. To get around this, modern parallel SAT Solvers use a mix of portfolio solving (where multiple solvers search the same space, but along different paths; This process can either be deterministic or stochastic), search space splitting, and load balancing in order to acheive parallelisation @martins_overview_2012. These methods are good enough that they will on average perform better than an equivalent sequential solvers but they suffer from poor overall core-utilisation, and memory access limitations when sharing clauses between threads.

== The Quadratic Unconstrained Binary Optimization Problem

TODO: Define QUBO with math
// WOW

One notable feature of QUBO problems is their suitability for quantum computers, specifically through a technique known as quantum annealing. Quantum annealing is a quantum computing approach designed to efficiently solve optimization problems like QUBO. It leverages the principles of quantum mechanics to encode the QUBO solution in qubits that find solutions by , making it potentially faster than classical computers for certain optimization tasks.

The intersection of QUBO problems and quantum computing has sparked significant research interest. This interest has also lead to an interest in solving QUBO problems using parallel computing, not only quantum computing. Various parallelization techniques have been developed for QUBO problems. Some of these techniques aim to simulate the quantum annealing process using parallel computing hardware, while others explore novel approaches to solve QUBO problems.

This project focuses on bridging the gap between SAT and QUBO to create a SAT solver that employs a reduction from SAT to QUBO as a means to enable the easier parallelisation, and potentially quantum SAT solving. Leveraging the substantial body of research surrounding implementing QUBO on quantum and parallel computers, and the work that has been done to investigate SAT reductions to QUBO, I aim to not only to create a parallel SAT solver, but also compare this approach with existing approaches.

Should the performance of the solver be good enough, I wish to submit this solver to the parallel track of the 2024 _#link("http://satcompetition.org/")[SAT Competition]_.

= Motivation

= Related Work

This project builds off a myriad of work in SAT reductions and in the field of Parallel QUBO.

= Description of the work

= Methodology

The solver will be evaluted using the #link("https://satcompetition.github.io/2023/downloads.html")[SAT Competition 2023 Parallel Track benchmarks]. This provides a set of comprehensive benchmarks over both real and theoretical problems, and a standard point of comparison with a swathe of modern parallel SAT solvers. The main metric that will be measured for the solver is time to solve.

= Design

The solver is designed to be flexible and allow for the implementation of many alternative backends allowing for different reduction and solving algorithms to easily be compared to one another. The general flow for solving a problem is shown below.

TODO: Image of solver pipeline

Input from the user is Processed into the SAT Problem instance. The SAT Problem is then reduced into a QUBO problem using one of the specified algorithms. We can then solve the reduced QUBO problem with another specified algorithm. Once we have a solution, we present it to the user.

== Reduction Algorithms

The plan is for the solver to implements 5 reduction algorithms:
- *Choi* @choi_different_2011 which uses a reduction from K-SAT to 3-SAT to MIS then finally QUBO
- A novel method which reduces K-SAT to Max-2-SAT and then to QUBO // Didn't implement in the end
- *Chancellor* @chancellor_direct_2016 which directly encodes problems in a Hamiltonian function that defines the QUBO Matrix. This is the current state-of-the-art method, and the resulting QUBO Matrices are notably smaller than that of Choi.
- *Nusslein 2022* @nuslein_algorithmic_2022 is similar to Chancellor, but is supposed to scale better for QUBO formulations where the resulting QUBO graph has a number of edges that is sub-quadratic i.e. $|E| = Theta(|V|)$
- *Nusslein 2023* @nuslein_solving_2023 is a from a preprint paper which is supposed to produce smaller QUBO matrices than Chancellor with similar characteristics

== QUBO Solving Algorithms

The plan is for the solver to implement 6 QUBO solving algorithms:
- *Simulated Annealing* // Implemented
- *Parallel Exhaustive Search* @tao_work-time_2020 // Implemented (CPU Only)
- *MOPSO* @fujimoto_solving_2021 // WIP
- *Momentum Annealing* @okuyama_binary_2019 // Not completed
- *Simulated Quantum Annealing* @volpe_integration_2023
- *Divers Adaptive Bulk Search* @nakano_diverse_2022 // This is the same as PES

TODO: High level overview on each of these methods and how they work
TODO: FDCC
TODO BASIN ANALYSIS
TODO BIG VALLEY STRUCTURE TSP
TODO CLusters

= Implementation

The Solver is written in Rust. Rust was chosen as it has good tools for abstraction of problems and a strong type system that makes it easy to encode problems in. Additionally it is fast and has really good tools for concurrency which will make coding the parallel sections much easier.

== Input/Output

Following typical SAT solver convention and in-line with the competition requirements, the solver uses the DIMACS Input format for inputing problem instances. Included in this project is a python file `generate_cnf.py` for generating random (mostly unsat) cnf instances.

The solver then outputs `SAT` with a model, `UNSAT`, or `UNKNOWN`.

== Problem & Solution Encoding

Problems are implemented as Rust traits which makes it easy to implement different problems which can be reduced into one another in definitive ways. There is an interfaces that all problems implement, `Problem`. There is also `Reduction` trait that provides an interface for problems to be reduced from one to another, and a `SolutionReductionReverser` for up-modelling solutions without requiring the whole original problem.
```rust
pub trait Problem<SolutionType, EvaluationType> {
    fn solve(&self) -> SolutionType;
    fn validate_solution(&self, solution: &SolutionType) -> EvaluationType;
}

pub trait Reduction<TSolutionType, TProblem: Problem<TSolutionType>, USolutionType, UProblem: Problem<USolutionType>> {
    fn reduce_problem(&self, problem: &TProblem) -> (UProblem, Box<dyn SolutionReductionReverser<TSolutionType, TProblem, USolutionType, UProblem>>);
}

pub trait SolutionReductionReverser<TSolutionType, TProblem: Problem<TSolutionType>, USolutionType, UProblem: Problem<USolutionType>> {
    fn reverse_reduce_solution(&self, solution: USolutionType) -> TSolutionType;
}

```
This makes it trivial to add new Problems to the solver as intermediary reductions for example the Choi KSAT to QUBO Reduction requires a reduction to 3SAT and then to Maximum Independent Set, which has an identical QUBO reperesntation
```rust
pub enum KSatToQuboReduction {
    Choi,
    Chancellor,
    // ...
}

impl Reduction<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboReduction {
    fn reduce_problem(&self, problem: &KSatProblem) -> QuboProblem {
        match self {
            KSatToQuboReduction::Choi => {
                let (threesat_problem, threesat_reverser) = KSatToThreeSatReduction.reduce_problem(problem);
                let (qubo_problem, qubo_reverser) = ThreeSatToQuboReduction::Choi.reduce_problem(threesat_problem);

                (qubo_problem, Box::new(KSatToQuboSolutionReductionReverser::Choi { threesat_reverser, qubo_reverser}))
            },
            // ...
        }
    }
}

impl SolutionReductionReverser<SatSolution, KSatProblem, QuboSolution, QuboProblem> for KSatToQuboSolutionReductionReverser {
    fn reverse_reduce_solution(&self, solution: QuboSolution) -> SatSolution {
        match self {
            KSatToQuboSolutionReductionReverser::Choi {qubo_reverser, threesat_reverser} => {
                let threesat_solution = qubo_reverser.reverse_reduce_solution(solution);

                threesat_reverser.reverse_reduce_solution(threesat_solution)
            },
            // ...
        }
    }
}
```
The underlying structure of problems can vary a lot, but rust allows us to be flexible with that as long as we implement the traits above. The QUBO problem uses a sparse matrix representation in order to improve memory efficiency. The problem also stores how it should be solved alongside it.
```rust
pub enum QuboProblemBackend {
    ParallelExhaustiveSearch,
    MopsoParallel,
    // ...
}
// ...
pub struct QuboProblem {
    problem_matrix: matrix::SparseMatrix<i32>,
    problem_backend: QuboProblemBackend
}
```
The SAT problem doesn't implement any solve function as it is tangential to this project. SAT instances are stored as the number of variables and a list of of lists of variables analagous to conjunctive normal form, each variable having whether it is negated (`true` for not negated, `false` for negated) and a which variable number it corresponds to.
```rust
pub struct KSatVariable(pub bool, pub usize);
pub struct KSatProblem(pub usize, Vec<Vec<KSatVariable>>);
```

Solutions can be any rust type which makes defining problems very flexible, for example when solving a SAT problem there are 3 possible outcomes, `SAT` or `UNSAT` or `UNKNOWN`, however for QUBO problems, there is only one form of solution which is the list of binary variables.
```rust
pub enum KSatSolution {
    Sat(Vec<bool>),
    Unsat,
    Unknown
}
// ...
pub struct QuboSolution(pub Vec<bool>);
```

== Reductions

TODO: Intricate overview on each of the reduction methods and how they work

== Solving

TODO: Intricate overview on each of the qubo backends methods and how they work

= Progress

== Project Management

Work on the project seems to be going smoothly. The work previously identified for Phase One has been completed and some of the Phase two work has begun to be tackled.

The current main challenge to implementation is the current inability to "un-reduce" solutions to their equivalent solution in the original problem. This might not be possible or may be expensive for some of the reductions.

Most of the work now has to focus on the implementation of the reductions and solvers outlined above and then benchmarking them to find the best possible combination of QUBO Solver and Sat Solver as these reductions and solutions have varying properties that may cause some solvers to be more efficient for some reductions than others.

One of the things I overlooked in my proposal was looking into other more traditional parallel SAT solvers for inspiration. This is something I plan to amend while I focus on implementing the other elements of the project.

*Phase One: Research [#strike()[November]December]*

- #strike()[Conduct a literature review to identify potential parallel algorithms for solving QUBO problems.]
- #strike()[Summarize the strengths and weaknesses of these algorithms.]
- #strike()[Create a comprehensive list of references in this area.]
- #strike()[Investigate existing methods for reducing SAT problems to QUBO.]
- #strike()[Examine the theoretical underpinnings of these reductions and their practical applicability.]
- #strike()[Identify key challenges and opportunities in this field.]
- #strike()[Compile the findings into an interim report discussing the chosen research direction.]
- Research and dissect other Sat solvers in the hopes of understanding how they simplify and split solution spaces efficiently

*Phase Two: Design & Implementation [December - January]*

- #strike()[Specify the solver's basic logic, data structures, and optimization techniques.]
- Analyse the reduction algorithm's compatibility with parallelization.
- Write the code for the SAT to QUBO reduction algorithms.
- Test the solver with sample SAT instances to validate its functionality.
- Code the parallel QUBO solving algorithms.
- Conduct initial performance tests for the parallel solver.
- Verify that the solver is always correct.
- Test the solver on various benchmark problems to assess its initial performance.

*Phase Three: Testing, Optimization, and Refinement [February - March]*

- Collect and analyze data on the solver's performance, including execution times and solution quality.
- Begin writing detailed documentation on the solver's mechanism, inputs, and outputs.
- Identify performance bottlenecks and areas for improvement.
- Implement optimization techniques to enhance the solver's efficiency.
- Conduct comparative tests to assess the impact of optimizations.
- Execute extensive testing on the refined solver using a diverse set of Max-SAT problems.
- Evaluate the solver's performance against existing solvers and quantum computing approaches.
- Record and analyze the results.
- Compile all the research, development, and testing findings into a comprehensive final report.

#let title = [Enhancing Max-SAT Solvers through QUBO Reduction]

#set text(
  font: "BlexSerif Nerd Font",
  size: 11pt
)
#set page(
  paper: "a4",
  margin: (x: 1.8cm, y: 1.5cm),
  header: locate(
    loc => if loc.page() > 1 {
        align(right)[*#title*]
    })
)

#align(horizon)[
  #align(center)[
  #text(17pt)[*#title*]
  \
  Daudi Wampamba Bandres\
  CS with AI \
  COMP4027 
  #link("mailto:psydw3@nottingham.ac.uk")
]]

#pagebreak()

#set page(
)

#set par(
  justify: true,
  leading: 0.52em,
)

= Background and Motivation

In the ever-evolving landscape of computing, both parallel computing and quantum computing hold significant promise, albeit in markedly distinct ways.

== Parallel & Quantum Computing

Parallel computing has become a pivotal technology in the modern world, playing an increasing role in recent years as the main method of improving the performance of classical computers. The increasing complexity of computational problems, has pushed industry and research to look to the efficient parallelisation of work as a solution to dealing with ever larger problems.

While parallel computing has gained prominence as a reliable method for enhancing classical computer performance, quantum computing remains in its experimental infancy, characterized by the unique abilities that quantum effects enable that offer the potential for groundbreaking speed-ups. These two computing paradigms have given rise to divergent strategies in algorithm development and problem-solving approaches.

== Quadratic Unconstrained Binary Optimization & Maximum Satisfiabililty

Quadratic Unconstrained Binary Optimization (QUBO) problems are a class of mathematical optimization problems that involve finding the best combination of binary variables to minimize or maximize a quadratic objective function. In a QUBO problem, each variable represents a binary decision. The objective function, which is quadratic in nature, quantifies the relationship between these binary variables and aims to optimize some real-world or abstract problem. Such problems have a wide range of applications, including portfolio optimization, scheduling, and circuit design, among others.

One notable feature of QUBO problems is their suitability for quantum computers, specifically through a technique known as quantum annealing. Quantum annealing is a quantum computing approach designed to efficiently solve optimization problems like QUBO. It leverages the principles of quantum mechanics to encode the QUBO solution in qubits that find solutions by , making it potentially faster than classical computers for certain optimization tasks.

The intersection of QUBO problems and quantum computing has sparked significant research interest. This interest has also lead to an interest in solving QUBO problems using parallel computing, not only quantum computing. Parallel computing involves breaking down a problem into smaller tasks that can be executed simultaneously, thus potentially speeding up computation. Various parallelization techniques have been developed in the context of QUBO problems. Some of these techniques aim to simulate the quantum annealing process on classical, non-quantum hardware, while others explore novel approaches that harness the unique computational power of quantum computers to directly solve QUBO problems.

The Maximum Satisfiability (Max-SAT) problem, an NP-Hard Optimization problem similarly to QUBO, has traditionally been approached with predominantly single-threaded solvers. Existing parallel techniques, centered around clause-sharing and portfolio-based methods, have faced challenges in achieving efficient core utilization and adapting to SIMD approaches for GPU acceleration.

The main output of this project is a fully featured Max-SAT solver that employs a reduction from Max-SAT to QUBO as a means to enable the simple parallelisation. Leveraging the substantial body of research surrounding QUBO, this approach aims to not only to create a parallel Max-SAT solver, but also compare this approach with existing apporaches, including classical, parallel, quantum apporaches.

= Aims and Objectives

The aim of this project is to produce a parallel and quantum-capable Max-SAT solver.
- Investigating existing solutions for a suitable reduction from Max-SAT to QUBO and developing a Max-SAT to QUBO reduction with a complexity that allows the parallel algorithm to maintain it's time complexity.
- Implementing and optimising an algorithm for solving QUBO in parallel based on the large body of existing work.
- Test and evaluate the performance of the resulting parallel Max-SAT solver, using the #underline(link("https://maxsat-evaluations.github.io/")[Max-SAT Evaluations set])
- Conduct an in-depth comparative evaluation of the efficacy of the solver in comparison to existing approaches, including other parallel algorithms and quantum computing approaches.

= External Aspect

This project carries a broad external applicability, with potential implications for a diverse range of stakeholders. The ability to efficiently parallelize Max-SAT solvers, extends its relevance beyond the research realm. It may find utility among industries grappling with complex optimization challenges, such as logistics and supply chain management, where the rapid and parallel resolution of Max-SAT problems is critical. Moreover, the project could draw interest from fields beyond computer science, including operations research and engineering.

#pagebreak()

= Work Plan

*Phase One: Research [November]*

- Conduct a literature review to identify potential parallel algorithms for solving QUBO problems.
- Summarize the strengths and weaknesses of these algorithms.
- Create a comprehensive list of references in this area.
- Investigate existing methods for reducing Max-SAT problems to QUBO.
- Examine the theoretical underpinnings of these reductions and their practical applicability.
- Identify key challenges and opportunities in this field.
- Compile the findings into an interim report discussing the chosen research direction.

*Phase Two: Design & Implementation [December - January]*

- Design a Max-SAT to QUBO reduction algorithm.
- Specify the algorithm's logic, data structures, and optimization techniques.
- Ensure the algorithm's compatibility with parallelization.
- Write the code for the Max-SAT to QUBO reduction algorithm.
- Test the algorithm with sample Max-SAT instances to validate its functionality.
- Plan the architecture of the parallel QUBO solver.
- Choose appropriate parallelization techniques (e.g., multi-threading, SIMD, GPU acceleration).
- Define how the solver will utilize multiple cores or processors effectively.
- Code the parallel QUBO solver following the design specifications.
- Ensure that the solver can handle a range of QUBO instances in parallel.
- Conduct initial performance tests for the parallel solver.
- Combine the Max-SAT to QUBO reduction algorithm with the parallel QUBO solver.
- Verify that the integrated solver functions correctly.
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

// #pagebreak()

= Bibliography

// #bibliography("proposal.bib")
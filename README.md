# Optimize-rs: An extensible framework for solving optimization problems

Otimize-rs is a framework for building solvers for optimisation problems, including a set of reference implementations of solvers and reductions.

## Setup

### Building from source

This project uses experimental Rust features. Be sure to install Rust Nightly with:

```
rustup install nightly
```

GPU features rely on SYCL. Install a valid DPCPP comliler and ensure `$DPCPP_HOME` is in the build environment.

This project is managed with cargo and by default builds and runs the cli. An example build and run is shown below.

```
cargo run -- test-sat.cnf
```

For help on how to use the CLI call the cli with the `-h, --help` flag.



#![feature(trait_alias)]

#[cfg(test)]
mod test;

mod helpers;
mod qubo;
pub mod solver;

pub use qubo::*;

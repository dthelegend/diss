#![feature(trait_alias)]

#[cfg(test)]
mod test;

mod helpers;
mod qubo;
pub mod solver;
pub mod record;

pub use qubo::*;

#![feature(trait_alias)]
use common::Reduction;
use qubo_problem::QuboProblem;
use sat_problem::KSatProblem;

pub mod chancellor;
pub mod choi;
pub mod nusslein;
pub mod nusslein23;

pub trait QuboToSatReduction = Reduction<KSatProblem, QuboProblem>;

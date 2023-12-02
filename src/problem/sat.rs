use std::fmt::Debug;
use std::fmt::Display;

pub mod ksat;
pub mod threesat;

pub enum SatSolution {
    Sat(Vec<bool>),
    Unsat,
    Unknown
}

// Conjunctive normal form KSAT Problem with N Variables
#[derive(Clone, Copy)]
pub struct SatVariable(pub bool, pub usize);

impl Debug for SatVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KSatVariable {}{}", if self.0 {""} else {"¬"}, self.1)
    }
}

impl Debug for SatSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(arg0) => write!(f, "Sat {}", arg0.iter().enumerate().map(|(i, x)| format!("{}{}", if *x {""} else {"¬"}, i)).collect::<Vec<String>>().join(" ")),
            Self::Unsat => write!(f, "Unsat"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Display for SatSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(_) => write!(f, "SATISFIABLE"),
            Self::Unsat => write!(f, "UNSATISFIABLE"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}
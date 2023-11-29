use std::{num, io, fmt::Debug};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

pub enum ErrorKind {
    IO(io::ErrorKind),
    ParseError(num::IntErrorKind),
    ClauseError,
    HeaderError,
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(arg0) => f.debug_tuple("IO").field(arg0).finish(),
            Self::ParseError(arg0) => f.debug_tuple("ParseError").field(arg0).finish(),
            Self::ClauseError => write!(f, "ClauseError"),
            Self::HeaderError => write!(f, "HeaderError"),
        }
    }
}
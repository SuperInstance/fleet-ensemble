use std::fmt;

/// Errors produced by fleet-ensemble.
#[derive(Debug)]
pub enum Error {
    /// No agents in the ensemble.
    EmptyEnsemble,
    /// Agent dimension doesn't match the budget dimension.
    DimensionMismatch { expected: usize, got: usize },
    /// Invalid ternary value (must be -1, 0, or 1).
    InvalidTernary(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEnsemble => write!(f, "ensemble is empty — no agents to coordinate"),
            Self::DimensionMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected}, got {got}")
            }
            Self::InvalidTernary(v) => {
                write!(f, "invalid ternary value: {v} (must be -1, 0, or 1)")
            }
        }
    }
}

impl std::error::Error for Error {}

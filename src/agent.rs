use serde::{Deserialize, Serialize};

/// An agent in the ensemble with a demand vector.
///
/// Each agent expresses what it wants to do as a vector of real-valued demands.
/// The Governor projects these demands onto the conservation surface so that
/// the ensemble's total action respects the shared budget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier for this agent.
    pub name: String,
    /// The agent's desired action vector (what it wants to do before constraint projection).
    pub demand: Vec<f64>,
}

impl Agent {
    /// Create a new agent with the given name and demand vector.
    ///
    /// The demand vector represents the agent's unconstrained desired action
    /// in each dimension of the shared conservation space.
    pub fn new(name: impl Into<String>, demand: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            demand,
        }
    }

    /// Number of dimensions in the demand vector.
    pub fn dim(&self) -> usize {
        self.demand.len()
    }

    /// Euclidean norm (L2) of the demand vector.
    pub fn norm(&self) -> f64 {
        self.demand.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Scale the demand vector by a scalar.
    pub fn scale(&self, factor: f64) -> Self {
        Self {
            name: self.name.clone(),
            demand: self.demand.iter().map(|x| x * factor).collect(),
        }
    }

    /// Add an offset vector to the demand.
    pub fn offset(&self, delta: &[f64]) -> Self {
        Self {
            name: self.name.clone(),
            demand: self.demand.iter().zip(delta.iter()).map(|(a, b)| a + b).collect(),
        }
    }
}

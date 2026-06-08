use serde::{Deserialize, Serialize};

/// The conservation budget that constrains the ensemble.
///
/// The budget specifies what the sum of all agents' projected demand vectors
/// must equal. For example:
/// - `ConservationBudget::zero_sum(n)` — total demand must be zero (like momentum conservation)
/// - `ConservationBudget::new(vec![100.0])` — total energy must equal 100
///
/// This is the multi-agent analog of a physical conservation law.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationBudget {
    /// The target sum vector that the ensemble must satisfy.
    pub target: Vec<f64>,
}

impl ConservationBudget {
    /// Create a budget with a specific target sum vector.
    ///
    /// After projection, `Σ agents[i].demand = target`.
    pub fn new(target: Vec<f64>) -> Self {
        Self { target }
    }

    /// Zero-sum budget: the total demand across all agents must be zero.
    ///
    /// This models strict conservation (e.g., if one agent gains energy,
    /// another must lose the same amount).
    pub fn zero_sum(dim: usize) -> Self {
        Self {
            target: vec![0.0; dim],
        }
    }

    /// Number of dimensions.
    pub fn dim(&self) -> usize {
        self.target.len()
    }

    /// Compute the violation: how far a set of demand vectors is from satisfying the budget.
    ///
    /// Returns the difference vector `Σ demands - target`.
    pub fn violation(&self, demands: &[Vec<f64>]) -> Vec<f64> {
        let n = self.target.len();
        let mut sum = vec![0.0; n];
        for d in demands {
            for (j, val) in d.iter().enumerate().take(n) {
                sum[j] += val;
            }
        }
        sum.iter().zip(self.target.iter()).map(|(s, t)| s - t).collect()
    }

    /// Check whether a set of demands satisfies the budget within tolerance.
    pub fn is_satisfied(&self, demands: &[Vec<f64>], tolerance: f64) -> bool {
        self.violation(demands).iter().all(|v| v.abs() <= tolerance)
    }
}

use crate::budget::ConservationBudget;
use crate::ensemble::Ensemble;
use crate::error::Error;

/// The conservation governor projects agents' demand vectors onto the constraint surface.
///
/// Given N agents each with a demand vector d_i ∈ ℝ^k and a conservation budget
/// with target t ∈ ℝ^k, the governor computes adjusted demands d_i' such that:
///
///   Σ_i d_i' = t
///
/// The projection is the minimum-norm correction: subtract the deficit equally
/// from all agents. This is equivalent to projecting onto the affine hyperplane
/// defined by the budget constraint.
#[derive(Debug, Clone)]
pub struct Governor {
    /// The conservation budget this governor enforces.
    pub budget: ConservationBudget,
}

impl Governor {
    /// Create a new governor enforcing the given budget.
    pub fn new(budget: ConservationBudget) -> Self {
        Self { budget }
    }

    /// Project all agents' demands onto the conservation surface.
    ///
    /// Returns adjusted demand vectors (one per agent) that satisfy the budget.
    /// The adjustment is the minimum-norm correction: the deficit is distributed
    /// equally across all agents.
    ///
    /// # Errors
    ///
    /// Returns `Error::EmptyEnsemble` if there are no agents.
    /// Returns `Error::DimensionMismatch` if agent dimensions don't match the budget.
    pub fn project(&self, ensemble: &Ensemble) -> Result<Vec<Vec<f64>>, Error> {
        if ensemble.is_empty() {
            return Err(Error::EmptyEnsemble);
        }

        let n = ensemble.len();
        let dim = self.budget.dim();

        for agent in ensemble.agents() {
            if agent.dim() != dim {
                return Err(Error::DimensionMismatch {
                    expected: dim,
                    got: agent.dim(),
                });
            }
        }

        // Compute the violation: Σ demands - target
        let total = ensemble.total_demand();
        let violation: Vec<f64> = total.iter().zip(self.budget.target.iter()).map(|(s, t)| s - t).collect();

        // Correction per agent: divide the violation equally
        let correction: Vec<f64> = violation.iter().map(|v| v / n as f64).collect();

        // Apply correction: adjusted_i = demand_i - correction
        let adjusted: Vec<Vec<f64>> = ensemble
            .agents()
            .iter()
            .map(|agent| {
                agent
                    .demand
                    .iter()
                    .zip(correction.iter())
                    .map(|(d, c)| d - c)
                    .collect()
            })
            .collect();

        Ok(adjusted)
    }

    /// Project and return the total violation before projection.
    ///
    /// Useful for diagnostics: how far was the ensemble from satisfying conservation?
    pub fn project_with_diagnostics(&self, ensemble: &Ensemble) -> Result<(Vec<Vec<f64>>, Vec<f64>), Error> {
        let total = ensemble.total_demand();
        let violation: Vec<f64> = total.iter().zip(self.budget.target.iter()).map(|(s, t)| s - t).collect();
        let adjusted = self.project(ensemble)?;
        Ok((adjusted, violation))
    }
}

//! # fleet-ensemble
//!
//! Conservation-governed multi-agent ensemble — agents coordinate under shared physical constraints.
//!
//! In real physics, conservation laws (energy, momentum, charge) constrain what individual
//! particles can do. This crate applies the same principle to agent coordination: each agent
//! acts independently, but a shared conservation budget ensures global coherence.
//!
//! ## Core Idea
//!
//! Every agent has a **demand vector** (what it wants to do) and the ensemble has a
//! **conservation budget** (total resources available). The `Governor` projects each agent's
//! demand onto the constraint surface, ensuring that the sum of all agents' actions respects
//! conservation laws — without requiring a central planner.
//!
//! ## Quick Start
//!
//! ```
//! use fleet_ensemble::{Ensemble, Agent, ConservationBudget, Governor};
//!
//! let mut ensemble = Ensemble::new();
//! ensemble.add_agent(Agent::new("alpha", vec![1.0, 0.5, -0.3]));
//! ensemble.add_agent(Agent::new("beta", vec![-0.2, 0.8, 0.1]));
//!
//! let budget = ConservationBudget::new(vec![0.0, 0.0, 0.0]); // zero-sum: total must be zero
//! let governor = Governor::new(budget);
//!
//! let adjusted = governor.project(&ensemble);
//! println!("Adjusted demands: {:?}", adjusted);
//! ```

pub mod agent;
pub mod budget;
pub mod governor;
pub mod error;
pub mod ensemble;
pub mod ternary;

pub use agent::Agent;
pub use budget::ConservationBudget;
pub use governor::Governor;
pub use error::Error;
pub use ensemble::Ensemble;
pub use ternary::TernaryVector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_sum_projection() {
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("a", vec![1.0, 0.0]));
        ens.add_agent(Agent::new("b", vec![0.0, 1.0]));

        let budget = ConservationBudget::zero_sum(2);
        let gov = Governor::new(budget);
        let adjusted = gov.project(&ens);

        // Sum of adjusted demands should be approximately zero
        let sum: Vec<f64> = (0..2)
            .map(|j| adjusted.iter().map(|a| a[j]).sum())
            .collect();
        assert!(sum.iter().all(|s| s.abs() < 1e-10), "sum should be ~0: {:?}", sum);
    }

    #[test]
    fn single_agent_conservation() {
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("solo", vec![3.0, -1.0, 2.0]));

        let budget = ConservationBudget::new(vec![0.0, 0.0, 0.0]);
        let gov = Governor::new(budget);
        let adjusted = gov.project(&ens);

        // Single agent: demand should be projected to zero
        assert!(adjusted[0].iter().all(|v| v.abs() < 1e-10));
    }

    #[test]
    fn budget_constraint_satisfied() {
        let mut ens = Ensemble::new();
        for i in 0..5 {
            ens.add_agent(Agent::new(
                &format!("agent_{}", i),
                vec![(i as f64 - 2.0) * 0.5, (i as f64 - 1.0) * 0.3],
            ));
        }

        let target = vec![1.0, -0.5];
        let budget = ConservationBudget::new(target.clone());
        let gov = Governor::new(budget);
        let adjusted = gov.project(&ens);

        let sum: Vec<f64> = (0..target.len())
            .map(|j| adjusted.iter().map(|a| a[j]).sum())
            .collect();

        for (got, expected) in sum.iter().zip(target.iter()) {
            assert!((got - expected).abs() < 1e-10, "budget not satisfied");
        }
    }

    #[test]
    fn ternary_vector_operations() {
        let tv = TernaryVector::new(vec![1, 0, -1, 1, 0, -1, 1, 1]);
        assert_eq!(tv.len(), 8);
        assert!((tv.density() - 0.625).abs() < 1e-10);
        assert!((tv.balance() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn ternary_to_notes() {
        let tv = TernaryVector::new(vec![1, 0, -1, 1]);
        let notes = tv.to_notes(60);
        assert_eq!(notes.len(), 5);
        assert_eq!(notes[0], 60);
        assert_eq!(notes[1], 64); // +4
        assert_eq!(notes[2], 64); // 0
        assert_eq!(notes[3], 60); // -4
        assert_eq!(notes[4], 64); // +4
    }

    #[test]
    fn ensemble_add_and_count() {
        let mut ens = Ensemble::new();
        assert_eq!(ens.len(), 0);
        ens.add_agent(Agent::new("a", vec![1.0]));
        ens.add_agent(Agent::new("b", vec![2.0]));
        assert_eq!(ens.len(), 2);
    }

    #[test]
    fn governor_preserves_mean_when_budget_matches() {
        // If agents already satisfy the budget, projection shouldn't change much
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("a", vec![1.0, -1.0]));
        ens.add_agent(Agent::new("b", vec![-1.0, 1.0]));

        let budget = ConservationBudget::zero_sum(2);
        let gov = Governor::new(budget);
        let adjusted = gov.project(&ens);

        // Already satisfies zero-sum: adjusted ≈ original
        for (adj, orig) in adjusted.iter().zip(ens.agents().iter()) {
            for (a, o) in adj.iter().zip(orig.demand.iter()) {
                assert!((a - o).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn error_display_works() {
        let e = Error::EmptyEnsemble;
        assert!(e.to_string().contains("empty"));
    }
}

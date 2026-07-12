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
//! let adjusted = governor.project(&ensemble).unwrap();
//! println!("Adjusted demands: {:?}", adjusted);
//! ```

pub mod agent;
pub mod budget;
pub mod ensemble;
pub mod error;
pub mod governor;
pub mod ternary;

pub use agent::Agent;
pub use budget::ConservationBudget;
pub use ensemble::Ensemble;
pub use error::Error;
pub use governor::Governor;
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
        let adjusted = gov.project(&ens).unwrap();

        // Sum of adjusted demands should be approximately zero
        let sum: Vec<f64> = (0..2)
            .map(|j| adjusted.iter().map(|a| a[j]).sum())
            .collect();
        assert!(
            sum.iter().all(|s| s.abs() < 1e-10),
            "sum should be ~0: {:?}",
            sum
        );
    }

    #[test]
    fn single_agent_conservation() {
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("solo", vec![3.0, -1.0, 2.0]));

        let budget = ConservationBudget::new(vec![0.0, 0.0, 0.0]);
        let gov = Governor::new(budget);
        let adjusted = gov.project(&ens).unwrap();

        // Single agent: demand should be projected to zero
        assert!(adjusted[0].iter().all(|v| v.abs() < 1e-10));
    }

    #[test]
    fn budget_constraint_satisfied() {
        let mut ens = Ensemble::new();
        for i in 0..5 {
            ens.add_agent(Agent::new(
                format!("agent_{}", i),
                vec![(i as f64 - 2.0) * 0.5, (i as f64 - 1.0) * 0.3],
            ));
        }

        let target = vec![1.0, -0.5];
        let budget = ConservationBudget::new(target.clone());
        let gov = Governor::new(budget);
        let adjusted = gov.project(&ens).unwrap();

        let sum: Vec<f64> = (0..target.len())
            .map(|j| adjusted.iter().map(|a| a[j]).sum())
            .collect();

        for (got, expected) in sum.iter().zip(target.iter()) {
            assert!((got - expected).abs() < 1e-10, "budget not satisfied");
        }
    }

    #[test]
    fn ternary_vector_operations() {
        // [1,0,-1,1,0,-1,1,1]: 6 of 8 entries are non-zero → density 0.75;
        // four +1 and two -1 → balance (4-2)/8 = 0.25.
        let tv = TernaryVector::new(vec![1, 0, -1, 1, 0, -1, 1, 1]);
        assert_eq!(tv.len(), 8);
        assert!((tv.density() - 0.75).abs() < 1e-10);
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
        let adjusted = gov.project(&ens).unwrap();

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

    #[test]
    fn ternary_try_new_rejects_invalid() {
        // The fallible constructor must surface Error::InvalidTernary instead of
        // panicking, and must name the first offending value.
        let err = TernaryVector::try_new(vec![1, 0, 2, -1]).unwrap_err();
        match err {
            Error::InvalidTernary(v) => assert_eq!(v, 2),
            other => panic!("expected InvalidTernary, got {other:?}"),
        }
        assert!(err.to_string().contains("2"));

        // Valid input still constructs cleanly.
        let tv = TernaryVector::try_new(vec![-1, 0, 1]).unwrap();
        assert_eq!(tv.values, vec![-1, 0, 1]);
    }

    #[test]
    fn total_demand_handles_ragged_dimensions_without_panicking() {
        // total_demand() is public and must not index out of bounds when agents
        // have differing lengths. It sizes to the widest agent and sums each
        // agent's full demand (narrower agents contribute zero to extra dims).
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("wide", vec![1.0, 2.0, 3.0]));
        ens.add_agent(Agent::new("narrow", vec![10.0])); // shorter than agents[0]

        let total = ens.total_demand();
        assert_eq!(total, vec![11.0, 2.0, 3.0]);

        // Reverse order: a narrow first agent, then a wide one — also must not panic.
        let mut ens2 = Ensemble::new();
        ens2.add_agent(Agent::new("narrow", vec![10.0]));
        ens2.add_agent(Agent::new("wide", vec![1.0, 2.0, 3.0]));
        assert_eq!(ens2.total_demand(), vec![11.0, 2.0, 3.0]);
    }

    #[test]
    fn governor_rejects_empty_ensemble() {
        let ens = Ensemble::new();
        let gov = Governor::new(ConservationBudget::zero_sum(2));
        match gov.project(&ens) {
            Err(Error::EmptyEnsemble) => {}
            other => panic!("expected EmptyEnsemble, got {other:?}"),
        }
    }

    #[test]
    fn governor_rejects_dimension_mismatch() {
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("a", vec![1.0, 0.0])); // dim 2
        ens.add_agent(Agent::new("b", vec![1.0, 0.0, 0.0])); // dim 3 != budget dim

        let gov = Governor::new(ConservationBudget::zero_sum(2));
        match gov.project(&ens) {
            Err(Error::DimensionMismatch { expected, got }) => {
                assert_eq!((expected, got), (2, 3));
            }
            other => panic!("expected DimensionMismatch, got {other:?}"),
        }
    }

    #[test]
    fn governor_diagnostics_returns_pre_projection_violation() {
        let mut ens = Ensemble::new();
        ens.add_agent(Agent::new("a", vec![5.0]));
        ens.add_agent(Agent::new("b", vec![5.0]));

        // Target total 10.0; raw sum is 10.0 ⇒ violation 0.0, no correction.
        let gov = Governor::new(ConservationBudget::new(vec![10.0]));
        let (adjusted, violation) = gov.project_with_diagnostics(&ens).unwrap();
        assert!(violation.iter().all(|v| v.abs() < 1e-12));
        assert!(adjusted.iter().all(|a| (a[0] - 5.0).abs() < 1e-12));

        // Non-trivial violation: target 0.0, raw sum 10.0 ⇒ violation 10.0,
        // corrected equally ⇒ each agent adjusted to 0.0.
        let gov0 = Governor::new(ConservationBudget::zero_sum(1));
        let (adjusted, violation) = gov0.project_with_diagnostics(&ens).unwrap();
        assert!((violation[0] - 10.0).abs() < 1e-12);
        assert!(adjusted.iter().all(|a| a[0].abs() < 1e-12));
    }

    #[test]
    fn agent_helpers_are_correct() {
        let a = Agent::new("a", vec![3.0, 4.0]);
        assert_eq!(a.dim(), 2);
        assert!((a.norm() - 5.0).abs() < 1e-12); // 3-4-5 triangle

        let doubled = a.scale(2.0);
        assert_eq!(doubled.demand, vec![6.0, 8.0]);
        assert_eq!(doubled.name, "a");

        let shifted = a.offset(&[1.0, -1.0]);
        assert_eq!(shifted.demand, vec![4.0, 3.0]);
    }

    #[test]
    fn budget_violation_and_satisfaction() {
        let budget = ConservationBudget::new(vec![1.0, -1.0]);
        assert_eq!(budget.dim(), 2);

        // Two agents: sums to [2.0, -2.0]; violation = [1.0, -1.0].
        let demands = vec![vec![1.5, -1.5], vec![0.5, -0.5]];
        assert_eq!(budget.violation(&demands), vec![1.0, -1.0]);
        assert!(!budget.is_satisfied(&demands, 0.5));
        assert!(budget.is_satisfied(&demands, 1.0)); // exactly at tolerance
    }

    #[test]
    fn ternary_to_demand_and_edges() {
        let tv = TernaryVector::new(vec![1, -1, 0]);
        assert_eq!(tv.to_demand(), vec![1.0, -1.0, 0.0]);

        // Empty vector edge cases must not divide by zero.
        let empty = TernaryVector::new(vec![]);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert!(empty.density().abs() < 1e-12);
        assert!(empty.balance().abs() < 1e-12);
        assert_eq!(empty.to_notes(60), vec![60]); // only the base pitch
    }
}

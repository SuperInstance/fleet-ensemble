//! Tutorial: Conservation-governed agent ensembles
//!
//! Shows how to create agent ensembles where resource demands
//! must satisfy conservation budgets (zero-sum or target-sum).

use fleet_ensemble::*;

fn main() {
    println!("=== Fleet Ensemble Tutorial ===\n");

    // Part 1: Create agents with resource demands
    println!("Part 1: Creating agents");
    let alice = Agent::new("alice", vec![2.0, 1.0, 0.5]);   // GPU, RAM, Network
    let bob = Agent::new("bob", vec![1.0, 3.0, 1.0]);
    let carol = Agent::new("carol", vec![0.5, 1.0, 2.0]);
    
    println!("  Alice: GPU=2.0, RAM=1.0, NET=0.5 (norm={:.2})", alice.norm());
    println!("  Bob:   GPU=1.0, RAM=3.0, NET=1.0 (norm={:.2})", bob.norm());
    println!("  Carol: GPU=0.5, RAM=1.0, NET=2.0 (norm={:.2})", carol.norm());
    println!();

    // Part 2: Build an ensemble
    println!("Part 2: Ensemble total demand");
    let mut ensemble = Ensemble::new();
    ensemble.add_agent(alice);
    ensemble.add_agent(bob);
    ensemble.add_agent(carol);
    
    let total = ensemble.total_demand();
    println!("  {} agents, total demand: {:?}", ensemble.len(), total);
    println!();

    // Part 3: Conservation budget (zero-sum)
    println!("Part 3: Zero-sum conservation budget");
    let budget = ConservationBudget::zero_sum(3); // all resources sum to zero
    let violation = budget.violation(&ensemble.demands().into_iter().cloned().collect::<Vec<_>>());
    let satisfied = budget.is_satisfied(
        &ensemble.demands().into_iter().cloned().collect::<Vec<_>>(), 
        0.01
    );
    println!("  Violation vector: {:?}", violation);
    println!("  Budget satisfied: {}", satisfied);
    println!("  (Not zero-sum — agents are demanding resources, not exchanging)");
    println!();

    // Part 4: Target-sum budget
    println!("Part 4: Target-sum budget (total resources = [4, 5, 3])");
    let target_budget = ConservationBudget::new(vec![4.0, 5.0, 3.5]);
    let target_sat = target_budget.is_satisfied(
        &ensemble.demands().into_iter().cloned().collect::<Vec<_>>(),
        0.5
    );
    println!("  Target: [4.0, 5.0, 3.5]");
    println!("  Actual: {:?}", total);
    println!("  Within tolerance (±0.5): {}", target_sat);
}

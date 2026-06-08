use fleet_ensemble::{Ensemble, Agent, ConservationBudget, Governor, TernaryVector};

fn main() {
    println!("=== Fleet Ensemble: Conservation-Governed Coordination ===\n");

    // Create an ensemble of agents with unconstrained demand vectors.
    // Each agent wants to do something — consume resources, move, act.
    let mut ensemble = Ensemble::new();
    ensemble.add_agent(Agent::new("alpha", vec![1.0, 0.5, -0.3]));
    ensemble.add_agent(Agent::new("beta", vec![-0.2, 0.8, 0.1]));
    ensemble.add_agent(Agent::new("gamma", vec![0.5, -0.3, 0.6]));

    println!("Agent demands (unconstrained):");
    for agent in ensemble.agents() {
        println!("  {}: {:?}", agent.name, agent.demand);
    }
    println!("Total demand: {:?}", ensemble.total_demand());

    // Create a zero-sum conservation budget: total must be zero in each dimension.
    // Like momentum conservation — every action has an equal and opposite reaction.
    let budget = ConservationBudget::zero_sum(3);
    let governor = Governor::new(budget);

    // Project demands onto the conservation surface
    let adjusted = governor.project(&ensemble).unwrap();

    println!("\nAdjusted demands (conservation-enforced):");
    let mut total = vec![0.0; 3];
    for (agent, adj) in ensemble.agents().iter().zip(adjusted.iter()) {
        println!("  {}: {:?}", agent.name, adj);
        for (j, v) in adj.iter().enumerate() {
            total[j] += v;
        }
    }
    println!("Total (should be ~0): {:?}", total);

    // Ternary vectors: discrete intent
    let tv = TernaryVector::new(vec![1, 0, -1, 1, 0, -1, 1, 1]);
    println!("\nTernary vector: {:?}", tv.values);
    println!("  Density: {:.1}%", tv.density() * 100.0);
    println!("  Balance: {:.2}", tv.balance());
    println!("  Notes (from C4): {:?}", tv.to_notes(60));

    // Custom budget example: total energy must equal 10
    let mut ens2 = Ensemble::new();
    ens2.add_agent(Agent::new("consumer_a", vec![7.0]));
    ens2.add_agent(Agent::new("consumer_b", vec![6.0]));

    let budget2 = ConservationBudget::new(vec![10.0]);
    let gov2 = Governor::new(budget2);
    let adjusted2 = gov2.project(&ens2).unwrap();

    println!("\nCustom budget (total = 10.0):");
    let sum: f64 = adjusted2.iter().map(|a| a[0]).sum();
    println!("  Adjusted: {:?}", adjusted2);
    println!("  Sum: {:.4} (target: 10.0)", sum);
}

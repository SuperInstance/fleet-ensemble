use crate::agent::Agent;

/// A multi-agent ensemble governed by shared conservation constraints.
///
/// Each agent has a demand vector representing its unconstrained desired action.
/// The ensemble is the collection that gets projected onto the conservation surface
/// by the `Governor`.
#[derive(Debug, Clone)]
pub struct Ensemble {
    agents: Vec<Agent>,
}

impl Ensemble {
    /// Create an empty ensemble.
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Add an agent to the ensemble.
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    /// Number of agents.
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Whether the ensemble is empty.
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Access the agents.
    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    /// Get the raw demand vectors.
    pub fn demands(&self) -> Vec<&Vec<f64>> {
        self.agents.iter().map(|a| &a.demand).collect()
    }

    /// Total demand (sum of all agents' demand vectors).
    pub fn total_demand(&self) -> Vec<f64> {
        if self.agents.is_empty() {
            return Vec::new();
        }
        let dim = self.agents[0].dim();
        let mut total = vec![0.0; dim];
        for agent in &self.agents {
            for (j, val) in agent.demand.iter().enumerate() {
                total[j] += val;
            }
        }
        total
    }
}

impl Default for Ensemble {
    fn default() -> Self {
        Self::new()
    }
}

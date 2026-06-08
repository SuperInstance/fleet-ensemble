/// Errors that can occur in the ensemble.
#[derive(Debug, Clone, PartialEq)]
pub enum EnsembleError {
    /// An agent with this id already exists.
    DuplicateAgent(String),
    /// No agent found with this id.
    AgentNotFound(String),
    /// Harmonic validation failed.
    HarmonyViolation(String),
}

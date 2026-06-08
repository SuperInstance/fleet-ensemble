/// Role an agent can play in the ensemble.
///
/// Lower value = higher priority when resolving conflicts.
/// The `Conductor` has absolute authority; the `Narrator` is the most permissive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum AgentRole {
    /// Supreme coordinator — overrides all others.
    Conductor = 0,
    /// Ensures rules are followed; high priority for veto.
    Guardian = 1,
    /// Evaluates aesthetic quality; can suggest alternatives.
    Critic = 2,
    /// Generates musical material (melodies, harmonies, rhythms).
    Builder = 3,
    /// Explores the sonic space; medium priority.
    Researcher = 4,
    /// Takes risks and experiments; lower priority.
    Explorer = 5,
    /// Blends parts together; medium-low priority.
    Integrator = 6,
    /// Provides narrative context; lowest musical priority.
    Narrator = 7,
}

impl AgentRole {
    /// Human-readable name of the role.
    pub fn name(self) -> &'static str {
        match self {
            Self::Conductor => "Conductor",
            Self::Guardian => "Guardian",
            Self::Critic => "Critic",
            Self::Builder => "Builder",
            Self::Researcher => "Researcher",
            Self::Explorer => "Explorer",
            Self::Integrator => "Integrator",
            Self::Narrator => "Narrator",
        }
    }

    /// Numeric priority (lower is higher priority).
    pub fn priority(self) -> u8 {
        self as u8
    }
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

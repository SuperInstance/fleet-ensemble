use crate::AgentRole;

/// A General MIDI instrument patch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstrumentPatch {
    /// Human-readable instrument name.
    pub name: String,
    /// General MIDI program number (0–127).
    pub program: u8,
    /// Bank select (usually 0 for GM).
    pub bank: u8,
}

impl InstrumentPatch {
    /// Create a new instrument patch.
    pub fn new(name: impl Into<String>, program: u8, bank: u8) -> Self {
        Self { name: name.into(), program, bank }
    }

    /// Acoustic Grand Piano (GM program 0).
    pub fn piano() -> Self { Self::new("Acoustic Grand Piano", 0, 0) }
    /// Ensemble Strings (GM program 48).
    pub fn strings() -> Self { Self::new("Ensemble Strings", 48, 0) }
    /// Electric Bass (finger) (GM program 33).
    pub fn bass() -> Self { Self::new("Electric Bass (finger)", 33, 0) }
    /// Flute (GM program 73).
    pub fn flute() -> Self { Self::new("Flute", 73, 0) }
    /// Drum kit (channel 10 convention).
    pub fn drums() -> Self { Self::new("Standard Drum Kit", 0, 120) }
}

impl std::fmt::Display for InstrumentPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (prog={} bank={})", self.name, self.program, self.bank)
    }
}

/// An agent registered with the ensemble.
///
/// Each agent has a unique id, a role that determines its priority,
/// a MIDI channel, an instrument patch, and mute/solo state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnsembleAgent {
    /// Unique identifier for this agent.
    pub id: String,
    /// The agent's role, which determines conflict-resolution priority.
    pub role: AgentRole,
    /// MIDI channel (0–15).
    pub channel: u8,
    /// The instrument this agent plays.
    pub instrument: InstrumentPatch,
    /// Whether this agent is muted (will not produce events).
    pub muted: bool,
    /// Whether this agent is in solo mode (only solo agents play).
    pub solo: bool,
}

impl EnsembleAgent {
    /// Create a new agent.
    pub fn new(id: String, role: AgentRole, channel: u8, instrument: InstrumentPatch) -> Self {
        Self { id, role, channel, instrument, muted: false, solo: false }
    }

    /// Convenience builder: set muted.
    pub fn muted(mut self, yes: bool) -> Self { self.muted = yes; self }
    /// Convenience builder: set solo.
    pub fn solo(mut self, yes: bool) -> Self { self.solo = yes; self }
}

impl std::fmt::Display for EnsembleAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} ch={} {}", self.role, self.id, self.channel, self.instrument)
    }
}

use crate::{AgentRole, MidiEvent};

/// A proposed MIDI event from an agent for a given tick.
#[derive(Debug, Clone)]
pub struct Proposal {
    /// The agent's role (used for priority).
    pub role: AgentRole,
    /// The agent's id.
    pub agent_id: String,
    /// The proposed MIDI event.
    pub event: MidiEvent,
}

/// Resolves conflicts when multiple agents propose events for the same tick.
///
/// Resolution strategy: events are sorted by role priority (lower = higher).
/// Among same-role agents, the first-registered wins (stable sort).
/// Optionally, events on the same channel that overlap in pitch are deduplicated.
#[derive(Debug, Clone, Default)]
pub struct ConflictResolver {
    /// If true, only the highest-priority agent per channel emits events.
    pub winner_takes_channel: bool,
}

impl ConflictResolver {
    /// Create a new resolver.
    pub fn new() -> Self {
        Self { winner_takes_channel: false }
    }

    /// Create a resolver where only the highest-priority agent per channel wins.
    pub fn winner_takes_all() -> Self {
        Self { winner_takes_channel: true }
    }

    /// Resolve a batch of proposals into a final event list.
    ///
    /// Steps:
    /// 1. Sort by role priority (ascending).
    /// 2. If `winner_takes_channel`, keep only the first proposal per channel.
    /// 3. Remove duplicate note-on events (same channel + pitch).
    pub fn resolve(&self, proposals: Vec<Proposal>) -> Vec<MidiEvent> {
        let mut sorted = proposals;
        sorted.sort_by_key(|p| p.role);

        if self.winner_takes_channel {
            let mut seen_channels = std::collections::HashSet::new();
            sorted.retain(|p| seen_channels.insert(p.event.channel));
        }

        // deduplicate same channel+note NoteOn events
        let mut seen = std::collections::HashSet::new();
        sorted.retain(|p| {
            if matches!(p.event.event_type, crate::MidiEventType::NoteOn { .. }) {
                seen.insert((p.event.channel, p.event.pitch()))
            } else {
                true
            }
        });

        sorted.into_iter().map(|p| p.event).collect()
    }
}

use crate::{
    EnsembleAgent, TempoTracker, KeySignature, HarmonicValidator,
    ConflictResolver, MidiEvent, EventStream, Proposal,
};

/// Operating mode of the ensemble.
///
/// Each mode changes how proposals are collected and resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum EnsembleMode {
    /// 1st species counterpoint: one action per tick, strict resolution.
    #[default]
    Synchronous,
    /// 2nd species: buffer proposals, emit on weak beats.
    Asynchronous,
    /// 5th species: free counterpoint, minimal resolution.
    Improvisational,
}

impl std::fmt::Display for EnsembleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Synchronous => write!(f, "Synchronous"),
            Self::Asynchronous => write!(f, "Asynchronous"),
            Self::Improvisational => write!(f, "Improvisational"),
        }
    }
}


/// The main ensemble coordinator.
///
/// Holds registered agents, tempo, key, and a harmonic validator.
/// Each call to `tick` collects proposals, resolves conflicts,
/// validates harmony, and emits events into an [`EventStream`].
#[derive(Debug, Clone)]
pub struct Ensemble {
    agents: Vec<EnsembleAgent>,
    tempo: TempoTracker,
    key: KeySignature,
    mode: EnsembleMode,
    resolver: ConflictResolver,
    validator: HarmonicValidator,
    stream: EventStream,
}

impl Ensemble {
    /// Create a new ensemble with the given tempo and key.
    pub fn new(tempo: TempoTracker, key: KeySignature) -> Self {
        Self {
            agents: Vec::new(),
            tempo,
            key,
            mode: EnsembleMode::Synchronous,
            resolver: ConflictResolver::new(),
            validator: HarmonicValidator::permissive(),
            stream: EventStream::new(),
        }
    }

    /// Set the ensemble mode.
    pub fn set_mode(&mut self, mode: EnsembleMode) {
        self.mode = mode;
    }

    /// Get the current mode.
    pub fn mode(&self) -> EnsembleMode {
        self.mode
    }

    /// Set the conflict resolver.
    pub fn set_resolver(&mut self, resolver: ConflictResolver) {
        self.resolver = resolver;
    }

    /// Set the harmonic validator.
    pub fn set_validator(&mut self, validator: HarmonicValidator) {
        self.validator = validator;
    }

    /// Get a reference to the tempo tracker.
    pub fn tempo(&self) -> &TempoTracker {
        &self.tempo
    }

    /// Get a mutable reference to the tempo tracker.
    pub fn tempo_mut(&mut self) -> &mut TempoTracker {
        &mut self.tempo
    }

    /// Get a reference to the key signature.
    pub fn key(&self) -> &KeySignature {
        &self.key
    }

    /// Get a mutable reference to the key signature.
    pub fn key_mut(&mut self) -> &mut KeySignature {
        &mut self.key
    }

    /// Register an agent. Returns `false` if an agent with the same id already exists.
    pub fn register(&mut self, agent: EnsembleAgent) -> bool {
        if self.agents.iter().any(|a| a.id == agent.id) {
            return false;
        }
        self.agents.push(agent);
        true
    }

    /// Unregister an agent by id. Returns true if removed.
    pub fn unregister(&mut self, id: &str) -> bool {
        let before = self.agents.len();
        self.agents.retain(|a| a.id != id);
        self.agents.len() != before
    }

    /// Get all registered agents.
    pub fn agents(&self) -> &[EnsembleAgent] {
        &self.agents
    }

    /// Find an agent by id.
    pub fn find_agent(&self, id: &str) -> Option<&EnsembleAgent> {
        self.agents.iter().find(|a| a.id == id)
    }

    /// Find an agent by id (mutable).
    pub fn find_agent_mut(&mut self, id: &str) -> Option<&mut EnsembleAgent> {
        self.agents.iter_mut().find(|a| a.id == id)
    }

    /// Mute/unmute an agent.
    pub fn set_muted(&mut self, id: &str, muted: bool) -> bool {
        if let Some(a) = self.find_agent_mut(id) {
            a.muted = muted;
            true
        } else {
            false
        }
    }

    /// Solo/unsolo an agent.
    pub fn set_solo(&mut self, id: &str, solo: bool) -> bool {
        if let Some(a) = self.find_agent_mut(id) {
            a.solo = solo;
            true
        } else {
            false
        }
    }

    /// Returns true if any agent is in solo mode.
    fn any_solo(&self) -> bool {
        self.agents.iter().any(|a| a.solo)
    }

    /// Returns the set of active agents (considering mute/solo).
    fn active_agents(&self) -> Vec<&EnsembleAgent> {
        let any_solo = self.any_solo();
        self.agents.iter().filter(|a| {
            if any_solo {
                a.solo && !a.muted
            } else {
                !a.muted
            }
        }).collect()
    }

    /// Collect proposals from active agents for the current tick.
    ///
    /// In a real system agents would produce proposals; here we provide
    /// a method for external callers to submit proposals for the current tick.
    pub fn collect_tick_proposals(&self, proposals: Vec<Proposal>) -> Vec<Proposal> {
        let active: std::collections::HashSet<&str> = self.active_agents().iter().map(|a| a.id.as_str()).collect();
        proposals.into_iter().filter(|p| active.contains(p.agent_id.as_str())).collect()
    }

    /// Run one tick: advance tempo, resolve proposals, validate, emit.
    ///
    /// Returns the events emitted this tick.
    pub fn tick(&mut self, proposals: Vec<Proposal>) -> Vec<MidiEvent> {
        let tick = self.tempo.tick;
        let filtered = self.collect_tick_proposals(proposals);

        let mut events = match self.mode {
            EnsembleMode::Synchronous => {
                // Strict: resolve all, take winners
                self.resolver.resolve(filtered)
            }
            EnsembleMode::Asynchronous => {
                // Buffer on strong beats, emit on weak beats
                if self.tempo.is_weak_beat() {
                    self.resolver.resolve(filtered)
                } else {
                    vec![]
                }
            }
            EnsembleMode::Improvisational => {
                // Free: just pass through, minimal filtering
                filtered.into_iter().map(|p| p.event).collect()
            }
        };

        // Validate harmony
        let note_events: Vec<u8> = events.iter()
            .filter(|e| e.is_note_on())
            .map(|e| e.pitch())
            .collect();
        let valid_notes = self.validator.validate(&note_events, &self.key);
        events.retain(|e| {
            if e.is_note_on() {
                valid_notes.contains(&e.pitch())
            } else {
                true
            }
        });

        // Stamp events with current tick
        for e in &mut events {
            e.tick = tick;
        }

        // Add to stream
        self.stream.extend(events.clone());
        self.tempo.advance();
        events
    }

    /// Get the output event stream.
    pub fn stream(&self) -> &EventStream {
        &self.stream
    }

    /// Get a mutable reference to the event stream.
    pub fn stream_mut(&mut self) -> &mut EventStream {
        &mut self.stream
    }

    /// Finalize the stream (sort by tick).
    pub fn finalize(&mut self) {
        self.stream.finalize();
    }

    /// Number of registered agents.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Reset the ensemble (clear stream, reset tempo).
    pub fn reset(&mut self) {
        self.tempo.reset();
        self.stream.clear();
    }
}

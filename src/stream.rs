/// Type of MIDI event.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MidiEventType {
    /// Note On: velocity 0–127.
    NoteOn { velocity: u8 },
    /// Note Off: velocity usually 0.
    NoteOff { velocity: u8 },
    /// Control Change.
    ControlChange { controller: u8, value: u8 },
    /// Program Change.
    ProgramChange { program: u8 },
}

/// A MIDI event with timing and channel information.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MidiEvent {
    /// Tick at which this event should occur.
    pub tick: u64,
    /// MIDI channel (0–15).
    pub channel: u8,
    /// Note or controller number (0–127).
    pub note: u8,
    /// The specific event type.
    pub event_type: MidiEventType,
}

impl MidiEvent {
    /// Create a Note On event.
    pub fn note_on(tick: u64, channel: u8, note: u8, velocity: u8) -> Self {
        Self { tick, channel, note, event_type: MidiEventType::NoteOn { velocity } }
    }

    /// Create a Note Off event.
    pub fn note_off(tick: u64, channel: u8, note: u8, velocity: u8) -> Self {
        Self { tick, channel, note, event_type: MidiEventType::NoteOff { velocity } }
    }

    /// Create a Control Change event.
    pub fn cc(tick: u64, channel: u8, controller: u8, value: u8) -> Self {
        Self { tick, channel, note: controller, event_type: MidiEventType::ControlChange { controller, value } }
    }

    /// Create a Program Change event.
    pub fn program_change(tick: u64, channel: u8, program: u8) -> Self {
        Self { tick, channel, note: 0, event_type: MidiEventType::ProgramChange { program } }
    }

    /// Pitch (note number) of this event.
    pub fn pitch(&self) -> u8 {
        self.note
    }

    /// Whether this is a Note On event.
    pub fn is_note_on(&self) -> bool {
        matches!(self.event_type, MidiEventType::NoteOn { .. })
    }

    /// Whether this is a Note Off event.
    pub fn is_note_off(&self) -> bool {
        matches!(self.event_type, MidiEventType::NoteOff { .. })
    }
}

impl std::fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.event_type {
            MidiEventType::NoteOn { velocity } => {
                write!(f, "t={} ch={} NOTE_ON  note={} vel={}", self.tick, self.channel, self.note, velocity)
            }
            MidiEventType::NoteOff { velocity } => {
                write!(f, "t={} ch={} NOTE_OFF note={} vel={}", self.tick, self.channel, self.note, velocity)
            }
            MidiEventType::ControlChange { controller, value } => {
                write!(f, "t={} ch={} CC cc={} val={}", self.tick, self.channel, controller, value)
            }
            MidiEventType::ProgramChange { program } => {
                write!(f, "t={} ch={} PC prog={}", self.tick, self.channel, program)
            }
        }
    }
}

/// A sorted stream of MIDI events ready for playback or export.
#[derive(Debug, Clone, Default)]
pub struct EventStream {
    events: Vec<MidiEvent>,
}

impl EventStream {
    /// Create an empty event stream.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an event (events need not be in order; `finalize` sorts).
    pub fn push(&mut self, event: MidiEvent) {
        self.events.push(event);
    }

    /// Push multiple events.
    pub fn extend(&mut self, events: impl IntoIterator<Item = MidiEvent>) {
        self.events.extend(events);
    }

    /// Sort events by tick.
    pub fn finalize(&mut self) {
        self.events.sort_by_key(|e| e.tick);
    }

    /// Get the sorted events.
    pub fn events(&self) -> &[MidiEvent] {
        &self.events
    }

    /// Number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether the stream is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Total duration in ticks (tick of the last event).
    pub fn duration_ticks(&self) -> u64 {
        self.events.iter().map(|e| e.tick).max().unwrap_or(0)
    }
}

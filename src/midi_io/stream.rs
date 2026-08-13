//! MIDI Stream — real-time MIDI I/O processing.
//!
//! Converts between raw MIDI events and CNS packets.
//! Uses `midly` for MIDI parsing and encoding.

use std::time::Instant;


/// A MIDI event with timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiEvent {
    /// Timestamp (microseconds since start).
    pub timestamp_us: u64,
    /// MIDI channel.
    pub channel: u8,
    /// Note number.
    pub note: u8,
    /// Velocity (0 = note off).
    pub velocity: u8,
}

impl MidiEvent {
    /// Is this a note-on event?
    pub fn is_note_on(&self) -> bool {
        self.velocity > 0
    }

    /// Is this a note-off event?
    pub fn is_note_off(&self) -> bool {
        self.velocity == 0
    }
}

/// MIDI stream processor — handles input/output buffering.
pub struct MidiStream {
    /// Start time for timestamp computation.
    start: Instant,

    /// Input buffer of pending MIDI events.
    input_buffer: Vec<MidiEvent>,

    /// Output buffer of MIDI events to be sent.
    output_buffer: Vec<MidiEvent>,

    /// Current MIDI tempo (microseconds per quarter note).
    tempo: u32,

    /// Current time signature numerator.
    ts_numerator: u8,

    /// Current time signature denominator.
    ts_denominator: u8,
}

impl MidiStream {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            input_buffer: Vec::with_capacity(256),
            output_buffer: Vec::with_capacity(256),
            tempo: 500_000, // 120 BPM default
            ts_numerator: 4,
            ts_denominator: 4,
        }
    }

    /// Get the current timestamp in microseconds.
    pub fn now_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }

    /// Push an incoming MIDI event.
    pub fn push_input(&mut self, event: MidiEvent) {
        self.input_buffer.push(event);
    }

    /// Queue an outgoing MIDI event.
    pub fn push_output(&mut self, event: MidiEvent) {
        self.output_buffer.push(event);
    }

    /// Drain pending input events.
    pub fn drain_input(&mut self) -> Vec<MidiEvent> {
        std::mem::take(&mut self.input_buffer)
    }

    /// Drain pending output events.
    pub fn drain_output(&mut self) -> Vec<MidiEvent> {
        std::mem::take(&mut self.output_buffer)
    }

    /// Set the current tempo (microseconds per quarter note).
    pub fn set_tempo(&mut self, tempo_us: u32) {
        self.tempo = tempo_us;
    }

    /// Get the current BPM.
    pub fn bpm(&self) -> f32 {
        60_000_000.0 / self.tempo as f32
    }

    /// Encode a MIDI note-on as raw bytes.
    pub fn encode_note_on(channel: u8, note: u8, velocity: u8) -> [u8; 3] {
        [
            0x90 | (channel & 0x0F),
            note & 0x7F,
            velocity & 0x7F,
        ]
    }

    /// Encode a MIDI note-off as raw bytes.
    pub fn encode_note_off(channel: u8, note: u8) -> [u8; 3] {
        [0x80 | (channel & 0x0F), note & 0x7F, 0x00]
    }

    /// Parse a raw MIDI byte triple into an event.
    pub fn parse_midi_bytes(data: &[u8; 3], timestamp_us: u64) -> Option<MidiEvent> {
        let status = data[0];
        let channel = status & 0x0F;
        let event_type = status & 0xF0;

        match event_type {
            0x90 => Some(MidiEvent {
                timestamp_us,
                channel,
                note: data[1],
                velocity: data[2],
            }),
            0x80 => Some(MidiEvent {
                timestamp_us,
                channel,
                note: data[1],
                velocity: 0, // note off
            }),
            _ => None,
        }
    }
}

impl Default for MidiStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_on_detection() {
        let event = MidiEvent {
            timestamp_us: 0,
            channel: 0,
            note: 60,
            velocity: 100,
        };
        assert!(event.is_note_on());
        assert!(!event.is_note_off());
    }

    #[test]
    fn note_off_detection() {
        let event = MidiEvent {
            timestamp_us: 0,
            channel: 0,
            note: 60,
            velocity: 0,
        };
        assert!(event.is_note_off());
        assert!(!event.is_note_on());
    }

    #[test]
    fn encode_note_on_correct_bytes() {
        let bytes = MidiStream::encode_note_on(0, 60, 100);
        assert_eq!(bytes[0], 0x90); // note on, channel 0
        assert_eq!(bytes[1], 60);   // middle C
        assert_eq!(bytes[2], 100);  // velocity
    }

    #[test]
    fn encode_note_off_correct_bytes() {
        let bytes = MidiStream::encode_note_off(3, 72);
        assert_eq!(bytes[0], 0x83); // note off, channel 3
        assert_eq!(bytes[1], 72);
        assert_eq!(bytes[2], 0x00);
    }

    #[test]
    fn parse_note_on_bytes() {
        let data = [0x90, 60, 100];
        let event = MidiStream::parse_midi_bytes(&data, 1000).unwrap();
        assert_eq!(event.channel, 0);
        assert_eq!(event.note, 60);
        assert_eq!(event.velocity, 100);
        assert!(event.is_note_on());
    }

    #[test]
    fn parse_note_off_bytes() {
        let data = [0x80, 60, 0];
        let event = MidiStream::parse_midi_bytes(&data, 2000).unwrap();
        assert_eq!(event.channel, 0);
        assert_eq!(event.note, 60);
        assert_eq!(event.velocity, 0);
        assert!(event.is_note_off());
    }

    #[test]
    fn parse_ignores_non_note_events() {
        let data = [0xB0, 0x07, 100]; // Control change
        assert!(MidiStream::parse_midi_bytes(&data, 0).is_none());
    }

    #[test]
    fn drain_input_clears_buffer() {
        let mut stream = MidiStream::new();
        stream.push_input(MidiEvent {
            timestamp_us: 0,
            channel: 0,
            note: 60,
            velocity: 100,
        });
        let drained = stream.drain_input();
        assert_eq!(drained.len(), 1);
        assert!(stream.drain_input().is_empty());
    }

    #[test]
    fn bpm_calculation_from_tempo() {
        let mut stream = MidiStream::new();
        stream.set_tempo(500_000); // 120 BPM
        approx::assert_relative_eq!(stream.bpm(), 120.0, epsilon = 0.1);

        stream.set_tempo(1_000_000); // 60 BPM
        approx::assert_relative_eq!(stream.bpm(), 60.0, epsilon = 0.1);

        stream.set_tempo(250_000); // 240 BPM
        approx::assert_relative_eq!(stream.bpm(), 240.0, epsilon = 0.1);
    }

    #[test]
    fn channel_masking_in_encode() {
        // Channel > 15 should be masked
        let bytes = MidiStream::encode_note_on(20, 60, 100); // 20 & 0x0F = 4
        assert_eq!(bytes[0], 0x94);
    }
}

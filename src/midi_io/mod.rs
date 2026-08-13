//! MIDI I/O module — real-time MIDI input and output.
//!
//! Handles MIDI stream processing, device discovery, and the bridge
//! between CNS packets and raw MIDI bytes.
//!
//! - [`stream`] — device-agnostic MIDI event/stream processing (always available)
//! - [`realtime`] — live hardware MIDI via `midir` (requires `realtime-midi` feature)

pub mod realtime;
pub mod stream;

pub use realtime::{MidiDeviceList, MidiError, RealtimeMidiInput, RealtimeMidiOutput};
pub use stream::{MidiEvent, MidiStream};

//! MIDI I/O module — real-time MIDI input and output.
//!
//! Handles MIDI stream processing, device discovery, and the bridge
//! between CNS packets and raw MIDI bytes.

pub mod stream;

pub use stream::{MidiEvent, MidiStream};

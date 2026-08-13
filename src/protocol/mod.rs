//! Protocol module — CNS (Controller Nervous System) packet types.
//!
//! All communication between Director and Instrument Agents flows as
//! CNS packets over the broadcast bus. Packets are small, unacknowledged,
//! and broadcast to all subscribers.

pub mod packets;

pub use packets::*;

/// Dimensionality of the JEPA embedding space.
/// Start with 256; will grow when the real JEPA encoder is trained.
pub const EMBEDDING_DIM: usize = 256;

/// Maximum number of instruments in a single ensemble.
pub const MAX_INSTRUMENTS: usize = 32;

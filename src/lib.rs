//! Fleet Ensemble — Agentic music performance system.
//!
//! Two top-level subsystems:
//! - [`director`] — the ensemble-level intelligence (tri-chamber mind)
//! - [`instrument`] — individual instrument agents with JEPA perception
//!
//! Communication flows over the CNS bus via [`protocol`] packets.

pub mod director;
pub mod instrument;
pub mod midi_io;
pub mod protocol;

pub use director::Director;
pub use instrument::InstrumentAgent;

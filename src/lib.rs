//! # fleet-ensemble
//!
//! Multi-agent music coordination engine — the conductor for the `fleet-midi` ecosystem.
//!
//! Agents register with roles, the ensemble maintains shared tempo/key/time signature,
//! queries agents each tick, resolves conflicts, checks harmonic validity, and emits
//! a coherent MIDI event stream.
//!
//! # Quick start
//!
//! ```
//! use fleet_ensemble::*;
//!
//! let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::new(0, Mode::Major));
//! let agent = EnsembleAgent::new("piano-1".into(), AgentRole::Builder, 0, InstrumentPatch::piano());
//! ens.register(agent);
//! ```

mod agent;
mod role;
mod tempo;
mod key;
mod resolver;
mod harmony;
mod stream;
mod ensemble;
mod error;

pub use agent::{EnsembleAgent, InstrumentPatch};
pub use role::AgentRole;
pub use tempo::TempoTracker;
pub use key::{KeySignature, Mode};
pub use resolver::{ConflictResolver, Proposal};
pub use harmony::HarmonicValidator;
pub use stream::{MidiEvent, MidiEventType, EventStream};
pub use ensemble::{Ensemble, EnsembleMode};
pub use error::EnsembleError;

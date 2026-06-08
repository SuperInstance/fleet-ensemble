# fleet-ensemble

[![crates.io](https://img.shields.io/crates/v/fleet-ensemble.svg)](https://crates.io/crates/fleet-ensemble)
[![docs.rs](https://docs.rs/fleet-ensemble/badge.svg)](https://docs.rs/fleet-ensemble)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Multi-agent music coordination engine — the conductor for collaborative music systems.**

`fleet-ensemble` manages a group of musical agents playing together. Each agent
registers with a role (leader, follower, builder, improvisor), the ensemble
maintains shared tempo/key/time signature, queries agents each tick, resolves
conflicts between proposals, checks harmonic validity, and emits a coherent
MIDI event stream.

## Features

- **Agent roles** — `Leader`, `Follower`, `Builder`, `Improvisor` with
  tick-priority scheduling based on role hierarchy
- **Tempo tracking** — `TempoTracker` with PPQN resolution, tick-to-beat
  conversion, and tempo change management
- **Key management** — `KeySignature` with `Mode` (Major/Minor), automatic
  scale derivation, and transposition
- **Conflict resolution** — `ConflictResolver` processes `Proposal` objects
  from agents, resolving pitch/class/rhythm conflicts by priority
- **Harmonic validation** — `HarmonicValidator` checks proposals against the
  current key for harmonic coherence
- **Event stream** — `EventStream` collects `MidiEvent`s (note on/off, CC,
  program change) into a time-ordered MIDI output
- **Ensemble modes** — `Synchronous`, `Async`, `Improvisational` modes
  controlling how agents interact per tick
- **Instrument patches** — `InstrumentPatch` presets (piano, bass, drums, etc.)
  with channel and program assignment

## Quick Start

```rust
use fleet_ensemble::*;

// Create an ensemble at 120 BPM, 480 PPQN, C major
let mut ens = Ensemble::new(
    TempoTracker::new(120.0, 480),
    KeySignature::new(0, Mode::Major),
);

// Register a piano agent
let agent = EnsembleAgent::new(
    "piano-1".into(),
    AgentRole::Builder,
    0,                          // MIDI channel
    InstrumentPatch::piano(),
);
ens.register(agent);

// Tick the ensemble
ens.tick().unwrap();

// Retrieve MIDI output
let events = ens.drain_events();
for e in &events {
    println!("{:?}", e);
}
```

## Ensemble Modes

```rust
// Synchronous — all agents play on the same beat
let mode = EnsembleMode::Synchronous;

// Async — agents play independently
let mode = EnsembleMode::Async;

// Improvisational — agents propose, resolver picks best
let mode = EnsembleMode::Improvisational;
```

## Module Overview

| Module | Description |
|---|---|
| `ensemble` | `Ensemble`, `EnsembleMode` — main coordination engine |
| `agent` | `EnsembleAgent`, `InstrumentPatch` — agent registration |
| `role` | `AgentRole` — Leader, Follower, Builder, Improvisor |
| `tempo` | `TempoTracker` — BPM and PPQN management |
| `key` | `KeySignature`, `Mode` — key/scale tracking |
| `resolver` | `ConflictResolver`, `Proposal` — conflict resolution |
| `harmony` | `HarmonicValidator` — harmonic correctness checks |
| `stream` | `MidiEvent`, `MidiEventType`, `EventStream` — MIDI output |
| `error` | `EnsembleError` — error types |

## Links

- [Documentation](https://docs.rs/fleet-ensemble)
- [Repository](https://github.com/nightshift-crates/fleet-ensemble)
- [Crates.io](https://crates.io/crates/fleet-ensemble)

## License

MIT

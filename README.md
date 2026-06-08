# fleet-ensemble

[![crates.io](https://img.shields.io/crates/v/fleet-ensemble.svg)](https://crates.io/crates/fleet-ensemble)
[![docs.rs](https://docs.rs/fleet-ensemble/badge.svg)](https://docs.rs/fleet-ensemble)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Problem

Multiple agents collaborating on music need coordination. But not all collaboration is the same:

- An orchestra needs a **conductor** — synchronous, everyone follows the same clock
- A jazz combo needs **cues** — asynchronous, agents signal each other
- An AI fleet needs **improvisation** — agents react to what they hear, no central control

Most multi-agent music systems pick one model. Real musical collaboration mixes all three. `fleet-ensemble` treats ensemble coordination as a spectrum from fully synchronized to fully autonomous, with formal models for each.

## The Three Modes

### Synchronous (Orchestra Mode)

A global clock drives all agents. Every beat, each agent produces output simultaneously. Good for:
- Drum machines with fixed patterns
- Sequenced backing tracks
- Any situation where timing must be sample-accurate

```rust
use fleet_ensemble::{Ensemble, SyncEnsemble};

let ensemble = SyncEnsemble::new(120.0, 4, 4); // 120 BPM, 4/4 time
ensemble.add_agent("bass", Box::new(bass_agent));
ensemble.add_agent("drums", Box::new(drums_agent));

// Tick-by-tick: all agents produce output simultaneously
for tick in 0..480 { // 4 measures of 120 ticks each
    let frame = ensemble.tick(tick);
    for (name, output) in frame {
        println!("{}: {:?}", name, output);
    }
}
```

### Asynchronous (Chamber Music Mode)

Agents run independently and communicate via messages. An agent can cue another ("your solo starts now"), pass motifs, or signal endings. Good for:
- Call-and-response patterns
- Leader/follower relationships
- Agents with different internal clocks

```rust
use fleet_ensemble::{AsyncEnsemble, Message};

let mut ensemble = AsyncEnsemble::new();
ensemble.add_agent("melody", Box::new(melody_agent));
ensemble.add_agent("harmony", Box::new(harmony_agent));

// Agents send messages to each other
ensemble.send("melody", Message::Motif(vec![60, 64, 67]));
ensemble.step(); // each agent processes one step independently
```

### Improvisational (Free Jazz Mode)

Agents listen to a shared "pool" of recent output and react to what they hear. No direct communication — influence is through the musical texture itself. Good for:
- Generative ambient music
- Emergent patterns from simple rules
- Situations where you don't know the structure in advance

```rust
use fleet_ensemble::{ImprovEnsemble, ListeningPool};

let mut ensemble = ImprovEnsemble::new(ListeningPool::new(16)); // 16-beat memory
ensemble.add_agent("agent-1", Box::new(agent_1));
ensemble.add_agent("agent-2", Box::new(agent_2));

for _ in 0..100 {
    let outputs = ensemble.step(); // each agent hears the pool and reacts
    ensemble.update_pool(&outputs); // pool evolves
}
```

## Role System

Agents can have roles that constrain their behavior:

```rust
use fleet_ensemble::{Role, AgentConfig};

let bass = AgentConfig::new("bass")
    .role(Role::Foundation)    // plays on beats 1 and 3
    .range(28..48)             // limited to bass register
    .max_density(0.5);         // plays at most 50% of available slots

let solo = AgentConfig::new("solo")
    .role(Role::Soloist)       // plays freely
    .range(60..84)             // mid-high register
    .priority(10);             // wins conflicts
```

## Conservation Across the Ensemble

The fleet conservation law (γ + H = constant) applies here: the total energy across all agents should remain bounded. The ensemble tracks:

```rust
let stats = ensemble.conservation_stats();
println!("Total energy: {:.2}", stats.total_energy);
println!("Entropy H: {:.4}", stats.entropy);
println!("γ + H = {:.4} (should be ~constant)", stats.gamma + stats.entropy);
```

## Module Map

| Module | What it does |
|---|---|
| `ensemble` | `Ensemble` trait — the base interface for all coordination modes |
| `sync` | `SyncEnsemble` — global clock, all agents tick together |
| `r#async` | `AsyncEnsemble` — message-passing between independent agents |
| `improv` | `ImprovEnsemble` — shared listening pool, no direct communication |
| `role` | `Role`, `AgentConfig` — role-based constraints |
| `message` | `Message` — inter-agent communication (motifs, cues, signals) |
| `pool` | `ListeningPool` — shared recent-output buffer for improv mode |
| `conservation` | Fleet-wide energy and entropy tracking |
| `agent` | `Agent` trait — implement this for your own musical agents |
| `error` | `EnsembleError` |

## Design Decisions

- **Three modes, not one**: Because real music uses all three. A jazz performance is synchronous (rhythm section locks), asynchronous (horns cue each other), and improvisational (everyone reacts to the texture) simultaneously.
- **Trait-based agents**: Your agent implements `Agent` with a single `tick()` or `react()` method. The ensemble handles coordination — your agent handles music.
- **Conservation tracking**: Stolen from the fleet's γ + H law. If the ensemble's energy grows without bound, something is wrong (one agent is dominating). Conservation as a health metric.

## Links

- [Documentation](https://docs.rs/fleet-ensemble)
- [Repository](https://github.com/SuperInstance/fleet-ensemble)
- [crates.io](https://crates.io/crates/fleet-ensemble)

## License

MIT

# fleet-ensemble

Conservation-governed multi-agent ensemble — agents coordinate under shared physical constraints.

## Core Idea

In real physics, conservation laws (energy, momentum, charge) constrain what individual
particles can do. This crate applies the same principle to agent coordination: each agent
acts independently, but a shared conservation budget ensures global coherence.

Every agent has a **demand vector** (what it wants to do) and the ensemble has a
**conservation budget** (total resources available). The `Governor` projects each agent's
demand onto the constraint surface, ensuring that the sum of all agents' actions respects
conservation laws — without requiring a central planner.

## Quick Start

```rust
use fleet_ensemble::{Ensemble, Agent, ConservationBudget, Governor};

let mut ensemble = Ensemble::new();
ensemble.add_agent(Agent::new("alpha", vec![1.0, 0.5, -0.3]));
ensemble.add_agent(Agent::new("beta", vec![-0.2, 0.8, 0.1]));

let budget = ConservationBudget::zero_sum(3); // dim must match the agents (3)
let governor = Governor::new(budget);

let adjusted = governor.project(&ensemble).unwrap();
```

## Status

Implemented and covered by tests:
- `Agent` demand vectors with `norm`/`scale`/`offset`
- `ConservationBudget` (zero-sum and target-sum) with `violation`/`is_satisfied`
- `Governor::project` — minimum-norm projection onto the conservation surface
  (Σ adjusted demands equals the target)
- `TernaryVector` discrete intent with density/balance/MIDI mapping
- Fallible `TernaryVector::try_new` returning `Error::InvalidTernary`

Scope: this is a **single-process, synchronous, in-memory** coordination
library. `Agent`s are data with demand vectors; the `Governor` projects them
onto a shared constraint surface. There is no concurrency, networking, or
distributed runtime — "multi-agent" refers to coordinating multiple demand
vectors under one conservation budget, not a fleet of autonomous processes.

## Verification

```sh
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## License

MIT

# Fleet Ensemble

**An agentic performance system where MIDI tracks become living instruments that coalesce under a director's feel.**

> *Like fine paint on liquid white base paint — each color finds its edge in relation to the others, not by command, but by resonance.*

---

## The Vision

Every MIDI track can be rendered as an **agentic instrument** — a player with its own perception, its own reflexes, its own voice. An **agentic director** performs the rendering synoptically, shaping the feel of the whole ensemble. The instruments align themselves to the director's feel the way a jazz quartet locks into a pocket — not by following a click track, but by [listening](https://en.wikipedia.org/wiki/Active_listening).

This is the [agentic compiler](https://en.wikipedia.org/wiki/Agentic_AI) pattern applied to music. Instead of compiling source code to machine code, we compile a musical score to a live performance. Each instrument is an agent. The director is the orchestrator. The canvas is the output.

The result is not a playback. It is a *happening* — a performance that is different every time, because the agents respond to each other and to the director's feel in real time. Like [Wittgenstein's language games](https://plato.stanford.edu/entries/wittgenstein/#LangGame), the meaning is in the playing, not in the score.

---

## Table of Contents

- [Architecture](#architecture)
- [The Canvas Metaphor](#the-canvas-metaphor)
- [The Director as Weather System](#the-director-as-weather-system)
- [The Agentic Compiler → Agentic Performer Analogy](#the-agentic-compiler--agentic-performer-analogy)
- [Relation to Fleet JEPA-MIDI](#relation-to-fleet-jepa-midi)
- [The Director is a JEPA](#the-director-is-a-jepa)
- [The Performer is Any Model](#the-performer-is-any-model)
- [The Instruments](#the-instruments)
- [Design Documents](#design-documents)
- [Status](#status)
- [License](#license)

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   AGENTIC DIRECTOR                           │
│   Hears the whole ensemble. Shapes the feel.                 │
│   "Lay back. Darker. More space. Push the bridge."           │
│   Operates on the ENSEMBLE level — not individual notes.     │
│                                                              │
│   ┌──────────────────────────────────────────────┐           │
│   │  TRI-CHAMBER ARCHITECTURE                    │           │
│   │  ┌─────────┐  ┌─────────┐  ┌─────────┐      │           │
│   │  │ ORACLE  │  │ MAESTRO │  │  PULSE  │      │           │
│   │  │ (LLM)   │  │ (Trained│  │ (Algo)  │      │           │
│   │  │ Phrase  │  │ Pulse   │  │ Sub-ms  │      │           │
│   │  │ level   │  │ level   │  │ level   │      │           │
│   │  └─────────┘  └─────────┘  └─────────┘      │           │
│   └──────────────────────────────────────────────┘           │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   │ FEEL_TILT packets
                   │ (7 parameters + per-instrument offsets)
                   │  ρ  epsilon  σ  τ  γ  λ  Φ
                   ▼
┌──────────────────────────────────────────────────────────────┐
│              AGENTIC INSTRUMENTS (one per track)              │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │  Piano   │  │   Bass   │  │  Drums   │  │  Guitar  │    │
│  │  Agent    │  │  Agent   │  │  Agent   │  │  Agent   │    │
│  │           │  │           │  │           │  │           │    │
│  │ Listens   │  │ Listens   │  │ Listens   │  │ Listens   │    │
│  │ to others │  │ to others │  │ to others │  │ to others │    │
│  │ Aligns to │  │ Aligns to │  │ Aligns to │  │ Aligns to │    │
│  │ director  │  │ director  │  │ director  │  │ director  │    │
│  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘    │
│        │              │              │              │          │
│        └──────────────┴──────┬───────┴──────────────┘          │
│                              │                                 │
│              CNS Protocol Bus                                 │
│              (embeddings, intents, drift, roles)              │
└──────────────────────────────┬─────────────────────────────────┘
                               │ MIDI events
                               ▼
                    ┌───────────────────┐
                    │   OUTPUT CANVAS    │
                    │                   │
                    │  The rendered      │
                    │  performance —     │
                    │  not a playback,   │
                    │  a happening       │
                    └───────────────────┘
```

The director operates across [five nested timescales](docs/director-design.md#6-timescales-the-directors-clocks): arc (minutes to hours), section (15-60 seconds), phrase (2-10 seconds), pulse (~125ms), and emergence detection (event-driven). Each timescale has its own cognitive chamber, from the [LLM Oracle](docs/director-design.md#41-the-oracle-llm--phrase-level-1-4-bars) that thinks in musical language to the algorithmic [Pulse](docs/director-design.md#43-the-pulse-algorithmic--sub-millisecond) that runs physics at sub-millisecond resolution.

---

## The Canvas Metaphor

[Liquid white base paint](https://en.wikipedia.org/wiki/Bob_Ross#Wet-on-wet_oil_painting) is the ground. Each instrument is a color poured onto the surface. The colors don't mix mechanically — they find their own edges through [surface tension](https://en.wikipedia.org/wiki/Surface_tension), [viscosity](https://en.wikipedia.org/wiki/Viscosity), density. The director doesn't tell each color where to go. **The director tilts the canvas.**

- **The base paint** is the [JEPA embedding space](https://en.wikipedia.org/wiki/Joint-Embedding_Predictive_Architecture) — the shared musical ground
- **Each color** is an instrument agent with its own timbre, register, and behavior
- **The tilt** is the director's feel parameters — the [seven-dimensional feel space](docs/director-design.md#22-the-seven-feel-parameters): pulse density, energy flux, harmonic tilt, temporal asymmetry, coupling pressure, risk appetite, articulation
- **The painting** is what emerges — unpredictable, alive, coalesced

The physics of paint on a tilted surface maps precisely to the math. The [Marangoni effect](https://en.wikipedia.org/wiki/Marangoni_effect) — flow driven by surface tension gradients — is the [harmonic rotation](docs/director-design.md#operation-1-harmonic-rotation-controlled-by-σ) `R_σ` in the SDE. [Brownian motion](https://en.wikipedia.org/wiki/Brownian_motion) of particles in the paint is the stochastic exploration `λ · dW`. The [drying time](https://en.wikipedia.org/wiki/Drying_of_paint) is the [exponential smoothing](docs/director-design.md#61-decoupling-from-the-grid) constant that prevents stepped transitions. This is not a loose analogy — it is the same mathematics.

---

## The Director as Weather System

The director does not conduct with a baton. It does not cue entrances. It does not correct pitch. **It is the air.** Two hundred years of orchestral tradition rested on a fatal hubris: that you could produce coherent collective beauty by prescribing trajectory for every discrete actor. This director does not command molecules. It sets the atmospheric conditions under which music condenses.

This architecture derives from [chaos theory](https://en.wikipedia.org/wiki/Chaos_theory), the original [Lorenz attractor](https://en.wikipedia.org/wiki/Lorenz_system) model of atmospheric convection, and the [Navier-Stokes equations](https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations) of fluid dynamics. Just as Navier-Stokes defines the rules of the medium — not the path of any single molecule — the director maintains only a global state vector of atmospheric feel parameters. No agent is told what to play. They feel the conditions of the space they play inside.

The feel parameters are atmospheric variables:

| Parameter | Symbol | Atmospheric Analog |
|-----------|--------|--------------------|
| [Pulse density](docs/director-design.md#22-the-seven-feel-parameters) | `ρ` | [Turbulence](https://en.wikipedia.org/wiki/Turbulence) — permitted shear between voices |
| [Energy flux](docs/director-design.md#22-the-seven-feel-parameters) | `ε` | [Thermal gradient](https://en.wikipedia.org/wiki/Lapse_rate) — energy differential across register |
| [Harmonic tilt](docs/director-design.md#22-the-seven-feel-parameters) | `σ` | [Barometric pressure](https://en.wikipedia.org/wiki/Atmospheric_pressure) — weight of silence between events |
| [Coupling pressure](docs/director-design.md#22-the-seven-feel-parameters) | `γ` | [Viscosity](https://en.wikipedia.org/wiki/Viscosity) — resistance to deviating from ensemble mean |
| [Risk appetite](docs/director-design.md#22-the-seven-feel-parameters) | `λ` | [Brownian motion](https://en.wikipedia.org/wiki/Brownian_motion) — baseline stochastic perturbation |

There is no master timeline. There is only a forecast. The director cannot tell you what will be played 17 bars from now. It can only tell you what the air will feel like then. You do not ask a storm to keep time. You stand inside it, and listen.

> *Full deep-dive: [Director Design §The Director as Weather System](docs/director-design.md#the-director-as-weather-system) and [§The Director as Spacetime Curvature](docs/director-design.md#the-director-as-spacetime-curvature).*

---

## The Agentic Compiler → Agentic Performer Analogy

| [Compiler Concept](https://en.wikipedia.org/wiki/Compiler) | Performance Concept | [Instrument Agent](docs/instrument-agent-design.md) Code Path |
|---|---|---|
| [Source code](https://en.wikipedia.org/wiki/Source_code) | Musical score / MIDI tracks | `score: Arc<ImmutableScore>` |
| [Lexer](https://en.wikipedia.org/wiki/Lexical_analysis) | JEPA pulse parser — reads the feel | `JEPA_ENCODER.forward(timeline)` |
| [Parser](https://en.wikipedia.org/wiki/Parsing) | Director interprets form and intent | `DirectorParams` broadcast |
| [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) | Ensemble arrangement — who plays what when | `intent_buffer: [NoteIntent; 128]` |
| [Optimization passes](https://en.wikipedia.org/wiki/Compiler_optimization) | Instrument agents adjust to each other | `AlignmentEngine` adjusts timing, dynamics, articulation |
| [Register allocation](https://en.wikipedia.org/wiki/Register_allocation) | Articulation assignment, voice leading deconfliction | `filter_notes()` + register conflict avoidance |
| [Code generation](https://en.wikipedia.org/wiki/Code_generation_(compiler)) | MIDI event generation (real-time) | `NoteRenderer` → MIDI Bus TX |
| [Linker](https://en.wikipedia.org/wiki/Linker_(computing)) | Mix — all instruments coalesce into one output | Ensemble cross-alignment resolves before audible output |
| [Runtime](https://en.wikipedia.org/wiki/Runtime_(program_lifecycle_phase)) | Continuous execution, drift correction, adaptation | The 1 kHz tick loop — forever [recompiling](docs/instrument-agent-design.md#8-compiler--performer-analogy) |
| Binary | The recording — frozen, but it was alive when it happened | Rendered audio file |

> **Critical insight:** Traditional MIDI sequencers are [ahead-of-time compilers](https://en.wikipedia.org/wiki/Ahead-of-time_compilation) — they produce the same binary every time. Fleet Ensemble agents are **[JIT compilers](https://en.wikipedia.org/wiki/Just-in-time_compilation) that recompile every millisecond while running.** The "binary" (rendered performance) is different every time, because the optimization passes respond to live ensemble conditions.

---

## Relation to Fleet JEPA-MIDI

| Fleet JEPA-MIDI | Fleet Ensemble |
|----------------|---------------|
| One soloist improvising | Full ensemble performing |
| LLM thinks, JEPA feels, algorithms execute | Director shapes, instruments align, canvas emerges |
| Single-track real-time generation | Multi-track agentic performance |
| Internal feedback loop | Inter-agent communication via [CNS protocol](docs/instrument-agent-design.md#4-communication-protocol) |

Fleet Ensemble uses Fleet JEPA-MIDI's [embedding space](https://en.wikipedia.org/wiki/Word_embedding) as its shared language. Each instrument agent has its own [JEPA reader](docs/instrument-agent-design.md#3-perception-pipeline--how-instruments-hear). The director operates on the ensemble-level embedding — the sum of all instruments' current states.

---

## The Director is a JEPA

The director is not an LLM. The director is not a conductor with a baton. **The director is a [JEPA](https://en.wikipedia.org/wiki/Joint-Embedding_Predictive_Architecture)** — a Joint Embedding Predictive Architecture ([LeCun, 2022](https://openreview.net/pdf?id=BZ5a1r-kVsf)) that perceives the whole ensemble's feel and outputs the tilt.

The JEPA director:
- **Perceives** every instrument's current state via their embeddings
- **Predicts** where the ensemble is heading
- **Outputs** the tilt — feel parameters that shape how instruments render:
  - **Tempo curve** — micro-adjustments to pulse (living tempo, not static BPM)
  - **Dynamic shape** — intensity envelope across the ensemble
  - **Color** — bright/dark, dense/sparse, active/still
  - **Weight** — which instrument carries the moment
  - **Space** — how much silence between events
- **Learns** what tilts produce what emergent behaviors — trained on real ensemble performances

This is perception-to-direction, not perception-to-language-to-direction. The JEPA doesn't describe the feel in words. It perceives the feel in its [latent space](https://en.wikipedia.org/wiki/Latent_space) and outputs directorial parameters directly.

---

## The Performer is Any Model

The performer is modular and pluggable. Any model that can take MIDI and render it as precise instructions for a music rendering system. Could be an [LLM](https://en.wikipedia.org/wiki/Large_language_model) that thinks in phrasing. Could be a [rules engine](https://en.wikipedia.org/wiki/Rule-based_system). Could be a trained [transformer](https://en.wikipedia.org/wiki/Transformer_(deep_learning_architecture)). The point: the performer renders MIDI with **intelligence and musicality** — not just notes on a page, but a real performance.

The performer receives:
- The MIDI score (what to play)
- The JEPA director's tilt (how to play it)
- Its own instrument's current state

The performer outputs:
- Rendered MIDI events with musicality — micro-timing, velocity shaping, phrasing, breath, space
- More than the score. The score made alive.

---

## The Instruments

Each instrument agent has [five modules](docs/instrument-agent-design.md#1-internal-architecture) and a shared internal clock running at 1 kHz:

1. **A voice** — its MIDI program, register, timbral preferences, polyphony limit
2. **A [JEPA reader](docs/instrument-agent-design.md#3-perception-pipeline--how-instruments-hear)** — perceives the ensemble state at pulse rate (62.5 Hz)
3. **A [reflex engine](docs/instrument-agent-design.md#module-responsibilities)** — fast algorithmic responses (<10ms), like spinal reflexes
4. **An [alignment module](docs/instrument-agent-design.md#5-alignment-mechanics)** — adjusts timing, dynamics, articulation, and note choice based on director feel + ensemble state
5. **A [listening module](docs/instrument-agent-design.md#module-responsibilities)** — hears other instruments via their embeddings + MIDI output

Instruments communicate through the **CNS protocol bus** — the same packet-based agent communication system the fleet uses. A MIDI note is a packet. A phrase is a pulse. A performance is a session.

### Three Personalities

Each instrument has a distinct [personality fingerprint](docs/instrument-agent-design.md#6-concrete-instrument-designs) — a set of behavioral parameters that determines how it responds to the ensemble:

- **[The Piano](docs/instrument-agent-design.md#61-piano-agent)** — The Accompanist-Poet (`alignment_gain: 0.25`). Listens more than it speaks. Drops notes to make space. Think [Herbie Hancock](https://en.wikipedia.org/wiki/Herbie_Hancock) in Miles's Second Quintet, [Bill Evans](https://en.wikipedia.org/wiki/Bill_Evans) in his trio.
- **[The Bass](docs/instrument-agent-design.md#62-bass-agent)** — The Anchor (`alignment_gain: 0.7`). Steady, foundational, almost never drops a root. Think [Ron Carter](https://en.wikipedia.org/wiki/Ron_Carter), [Paul Chambers](https://en.wikipedia.org/wiki/Paul_Chambers).
- **[The Drums](docs/instrument-agent-design.md#63-drum-agent)** — The Grid Incarnate (`alignment_gain: 0.9`). Defines the time. Everyone else adjusts to the drums. Think [Tony Williams](https://en.wikipedia.org/wiki/Tony_Williams_(drummer)) at 19 with Miles, [Elvin Jones](https://en.wikipedia.org/wiki/Elvin_Jones) with Coltrane.

---

## Design Documents

- **[Director Design](docs/director-design.md)** — The agentic director's perception, feel space, tri-chamber architecture, emergence detection, mathematical formalism, and operational modes. Start here.
- **[Instrument Agent Design](docs/instrument-agent-design.md)** — The engineering spec for a single instrument agent: internal architecture, perception pipeline, communication protocol, alignment mechanics, concrete instrument designs, and training pipeline.

---

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.75+ (2021 edition)
- [fleet-gateway](https://github.com/SuperInstance/fleet-gateway) for LLM oracle calls
- A trained JEPA encoder from [fleet-jepa-midi](https://github.com/SuperInstance/fleet-jepa-midi)
- MIDI input/output via [virtual MIDI port](https://en.wikipedia.org/wiki/MIDI) or file

### Build

```bash
git clone https://github.com/SuperInstance/fleet-ensemble.git
cd fleet-ensemble
cargo build --release
```

### Run

```bash
# Start the ensemble with a MIDI score
./target/release/fleet-ensemble --score path/to/score.mid --bpm 120

# With specific instruments
./target/release/fleet-ensemble \
  --score path/to/score.mid \
  --instruments piano,bass,drums \
  --bpm 120
```

---

## Key Concepts

### The Seven Feel Parameters

The director communicates with instruments through a seven-dimensional [feel space](docs/director-design.md#22-the-seven-feel-parameters). Each parameter is a continuous value that the director adjusts in real-time:

| Parameter | Symbol | Range | What It Controls |
|-----------|--------|-------|------------------|
| **Pulse density** | ρ | [0, 1] | How many notes per pulse — sparse vs. dense |
| **Energy flux** | ε | [0, 1] | Dynamic intensity — quiet vs. loud |
| **Harmonic tilt** | σ | [-1, 1] | Brightness — dark/low vs. bright/high |
| **Temporal asymmetry** | τ | [-1, 1] | Time feel — behind vs. ahead of the beat |
| **Coupling pressure** | γ | [0, 1] | How much instruments should align with each other |
| **Risk appetite** | λ | [0, 1] | How much stochastic exploration is allowed |
| **Articulation** | Φ | [0, 1] | Note shape — staccato vs. legato |

These parameters are sent as `FEEL_TILT` packets on the [CNS protocol bus](https://github.com/SuperInstance/cns-bridge). Each instrument receives the global tilt plus per-instrument offsets.

### The Tri-Chamber Architecture

The director doesn't use a single brain. It uses three cognitive chambers, each operating at a different timescale:

1. **[Oracle (LLM)](docs/director-design.md#41-the-oracle-llm--phrase-level-1-4-bars)** — Thinks in musical language ("build tension," "quote the bridge"). Called every 1-4 bars. This is where musical knowledge lives.

2. **[Maestro (Trained Model)](docs/director-design.md#42-the-maestro-trained--pulse-level-125ms)** — A trained [neural network](https://en.wikipedia.org/wiki/Neural_network) that perceives the ensemble's current state at pulse rate. This is where musical feel lives — the JEPA.

3. **[Pulse (Algorithmic)](docs/director-design.md#43-the-pulse-algorithmic--sub-millisecond)** — Pure math running at sub-millisecond resolution. Physics-based models of timing, dynamics, and articulation. This is where precision lives.

### Alignment Mechanics

Each instrument has an [alignment engine](docs/instrument-agent-design.md#5-alignment-mechanics) that adjusts its playing based on the director's feel parameters and the ensemble's current state. The alignment gain determines how strongly an instrument follows the director vs. plays its own thing:

- **Low alignment gain (0.25)** — Independent, conversational. Like [Herbie Hancock](https://en.wikipedia.org/wiki/Herbie_Hancock) comping behind a soloist — drops notes, leaves space, responds.
- **High alignment gain (0.9)** — Tight, foundational. Like [Tony Williams](https://en.wikipedia.org/wiki/Tony_Williams_(drummer)) driving the band — defines the time, everyone else adjusts.

The alignment engine uses [stochastic differential equations (SDEs)](https://en.wikipedia.org/wiki/Stochastic_differential_equation) to smoothly interpolate between director-guided and self-directed behavior:

$$dX_t = \gamma (\mu_{director} - X_t)\,dt + \lambda\,dW_t$$

where $\gamma$ is the coupling pressure, $\mu_{director}$ is the director's target, and $\lambda \cdot dW_t$ is [Brownian motion](https://en.wikipedia.org/wiki/Brownian_motion) exploration.

---

## API Reference

### Director

```rust
use fleet_ensemble::Director;

let mut director = Director::new()
    .jepa_encoder("weights/jepa_encoder.pt")?
    .gateway("http://127.0.0.1:8787/v1")
    .feel_space(FeelSpace::default());

// Run the director loop
director.run(&score, &mut midi_output).await?;
```

### Instrument Agent

```rust
use fleet_ensemble::InstrumentAgent;

let piano = InstrumentAgent::new("piano")
    .voice(Voice::piano())
    .alignment_gain(0.25)      // conversational
    .reflex_latency(Duration::from_millis(10));

piano.run(&mut ensemble_bus).await?;
```

### CNS Protocol Bus

Instruments and the director communicate via the [CNS protocol bus](https://github.com/SuperInstance/cns-bridge):

```rust
// Director broadcasts feel parameters
bus.broadcast(FeelTilt {
    rho: 0.6, epsilon: 0.7, sigma: 0.3,
    tau: -0.1, gamma: 0.5, lambda: 0.3, phi: 0.6,
    per_instrument: HashMap::from([
        ("piano", Offset { rho: -0.1, ..Default::default() }),
        ("drums", Offset { tau: 0.05, ..Default::default() }),
    ]),
}).await?;

// Instrument responds
let tilt = bus.receive().await?;
let adjusted_notes = instrument.align(notes, &tilt);
```

---

## Configuration

### Director Configuration

```toml
[director]
tempo = 120
pulse_rate_hz = 8          # 16th notes at 120 BPM
phrase_bars = 4            # LLM called every 4 bars

[director.oracle]
model = "deepseek-chat"
gateway = "http://127.0.0.1:8787/v1"
max_directives = 5

[director.maestro]
encoder_path = "weights/jepa_encoder.pt"
smoothing = 0.8            # EMA smoothing factor

[director.pulse]
tick_rate_hz = 1000        # 1 kHz internal clock
```

### Instrument Configuration

```toml
[instruments.piano]
program = 0                # GM Acoustic Grand Piano
alignment_gain = 0.25
polyphony = 8
register_range = [21, 108] # A0 to C8

[instruments.bass]
program = 33               # GM Electric Bass
alignment_gain = 0.7
polyphony = 1
register_range = [28, 60]

[instruments.drums]
program = 0                # drums use channel 10
alignment_gain = 0.9
is_drum = true
```

---

## Testing

```bash
cargo test        # Unit tests: alignment math, CNS protocol, voice config
cargo test --test integration  # Multi-agent integration tests
cargo bench      # Performance: alignment latency, pulse tick timing
```

Tests verify:
- Alignment engine produces valid musical output under all feel parameter combinations
- CNS protocol packets are correctly serialized and deserialized
- Instrument agents respect polyphony limits and register ranges
- Director's tri-chamber switches correctly between Oracle/Maestro/Pulse
- 1 kHz tick loop maintains timing under load

---

## Deployment

### Local Real-Time

Fleet Ensemble runs locally — the 1 kHz tick loop and sub-millisecond algorithmic engine demand it. The LLM Oracle can be remote (via fleet-gateway), but everything else must be local.

### Systemd Service

```bash
[Unit]
Description=Fleet Ensemble — Agentic Performance System
After=fleet-gateway.service

[Service]
Type=simple
WorkingDirectory=%h/projects/fleet-ensemble
ExecStart=%h/projects/fleet-ensemble/target/release/fleet-ensemble --score %h/scores/current.mid
Restart=always
MemoryMax=1G

[Install]
WantedBy=default.target
```

### DAW Integration

Connect Fleet Ensemble to your [DAW](https://en.wikipedia.org/wiki/Digital_audio_workstation) via [virtual MIDI ports](https://help.ableton.com/hc/en-us/articles/209071169-Creating-and-using-virtual-MIDI-ports):

1. Create a virtual MIDI input (e.g., `ensemble_in`)
2. Create a virtual MIDI output (e.g., `ensemble_out`)
3. Point Fleet Ensemble at those ports
4. Route MIDI tracks in your DAW to/from those ports

---

## Further Reading — Curated Bibliography

### For Developers

- [Director Design](docs/director-design.md) — the full director architecture specification
- [Instrument Agent Design](docs/instrument-agent-design.md) — the instrument agent engineering spec
- [CNS Protocol (cns-bridge)](https://github.com/SuperInstance/cns-bridge) — the communication bus
- [fleet-jepa-midi Design Docs](https://github.com/SuperInstance/fleet-jepa-midi/tree/main/docs) — the JEPA encoder this system uses
- [MIDI 1.0 Specification](https://www.midi.org/specifications-old/item/the-midi-1-0-specification)
- [General MIDI Standard](https://en.wikipedia.org/wiki/General_MIDI) — instrument program numbers
- [Stochastic Differential Equations in Rust](https://docs.rs/rand/latest/rand/) — RNG for SDE solving

### For Musicians

- [Miles Davis Second Quintet (Wikipedia)](https://en.wikipedia.org/wiki/Miles_Davis_Quintet) — the musical reference for ensemble interaction
- [Herbie Hancock](https://en.wikipedia.org/wiki/Herbie_Hancock) — conversational comping
- [Bill Evans Trio](https://en.wikipedia.org/wiki/Bill_Evans#Trio) — democratic interplay
- [Tony Williams](https://en.wikipedia.org/wiki/Tony_Williams_(drummer)) — push and pull of time
- [Elvin Jones](https://en.wikipedia.org/wiki/Elvin_Jones) — polyrhythmic foundation
- [Ron Carter](https://en.wikipedia.org/wiki/Ron_Carter) — the anchor
- [The Jazz Process](https://www.amazon.com/Jazz-Process-Adrian-Cho/dp/0321638354) by Adrian Cho — collaboration lessons from jazz

### For Mathematicians

- [Stochastic Differential Equations (Wikipedia)](https://en.wikipedia.org/wiki/Stochastic_differential_equation) — the math of the alignment engine
- [Brownian Motion (Wikipedia)](https://en.wikipedia.org/wiki/Brownian_motion) — the stochastic exploration term
- [Lorenz System (Wikipedia)](https://en.wikipedia.org/wiki/Lorenz_system) — atmospheric chaos theory inspiring the director
- [Navier-Stokes Equations (Wikipedia)](https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations) — fluid dynamics as ensemble metaphor
- [Marangoni Effect (Wikipedia)](https://en.wikipedia.org/wiki/Marangoni_effect) — surface tension gradients = harmonic rotation
- [Exponential Moving Average (Wikipedia)](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) — smoothing and decoupling
- [Game Theory (Wikipedia)](https://en.wikipedia.org/wiki/Game_theory) — multi-agent alignment as cooperative game

### For Engineers

- [SDE Numerical Methods (Euler-Maruyama)](https://en.wikipedia.org/wiki/Euler%E2%80%93Maruyama_method) — solving SDEs in discrete time
- [Real-Time Audio Programming](https://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nobody) — why the 1 kHz tick matters
- [MIDI Clock vs. MIDI Time Code](https://en.wikipedia.org/wiki/MIDI_clock) — timing synchronization
- [Jitter Buffer (Wikipedia)](https://en.wikipedia.org/wiki/Jitter_buffer) — handling timing variability
- [Systemd Services](https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html) — production deployment

### For Educators

- [Agentic AI (Wikipedia)](https://en.wikipedia.org/wiki/Agentic_AI) — the paradigm this system exemplifies
- [Compiler Theory (Wikipedia)](https://en.wikipedia.org/wiki/Compiler) — the compiler → performer analogy
- [Emergence (Wikipedia)](https://en.wikipedia.org/wiki/Emergence) — how complex behavior arises from simple agents
- [Language Games (Wittgenstein)](https://plato.stanford.edu/entries/wittgenstein/#LangGame) — meaning is in the playing
- [Bob Ross Wet-on-Wet Technique](https://en.wikipedia.org/wiki/Bob_Ross#Wet-on-wet_oil_painting) — the canvas metaphor

---

## Status

**Concept phase.** Repo created Aug 13, 2026. Design in progress. Architecture is fully specified; implementation begins next.

---

## License

MIT

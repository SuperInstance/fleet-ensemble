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

## Status

**Concept phase.** Repo created Aug 13, 2026. Design in progress. Architecture is fully specified; implementation begins next.

---

## License

MIT

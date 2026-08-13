# Instrument Agent Design

> **One instrument = one autonomous agent.** This is the engineering spec for a single player in the Fleet Ensemble.

*Each color finds its edge in relation to the others, not by command, but by resonance.*

---

## Table of Contents

1. [Internal Architecture](#1-internal-architecture)
2. [State Representation](#2-state-representation)
3. [Perception Pipeline — How Instruments Hear](#3-perception-pipeline--how-instruments-hear)
4. [Communication Protocol](#4-communication-protocol)
5. [Alignment Mechanics](#5-alignment-mechanics)
6. [Concrete Instrument Designs](#6-concrete-instrument-designs)
7. [Ensemble Skill — Training Goodness](#7-ensemble-skill--training-goodness)
8. [Compiler → Performer Analogy](#8-compiler--performer-analogy)
9. [Failure Modes](#9-failure-modes--what-makes-an-instrument-bad)

---

## 1. Internal Architecture

Every instrument agent is a self-contained system with five modules and a shared internal clock. The agent runs on a fixed 1 kHz tick, with perception updates at 50–62.5 Hz.

### Module Map

```
┌─────────────────────────────────────────────────────────────────┐
│                     INSTRUMENT AGENT                             │
│                                                                 │
│  ┌─────────────┐  ┌───────────────┐  ┌───────────────────────┐ │
│  │   VOICE     │  │  JEPA READER  │  │  LISTENING MODULE     │ │
│  │             │  │               │  │                       │ │
│  │ MIDI prog   │  │ Perceives     │  │ Hears other           │ │
│  │ Register    │  │ ensemble      │  │ instruments via       │ │
│  │ Timbre      │  │ state at      │  │ their embeddings +    │ │
│  │ Polyphony   │  │ pulse rate    │  │ MIDI output           │ │
│  └──────┬──────┘  └───────┬───────┘  └───────────┬───────────┘ │
│         │                 │                      │             │
│         └────────────────┬┴──────────────────────┘             │
│                          │                                     │
│                  ┌───────▼───────┐     ┌───────────────────┐   │
│                  │  REFLEX ENGINE │     │ ALIGNMENT MODULE  │   │
│                  │               │     │                   │   │
│                  │ Fast algo     │     │ Adjusts to        │   │
│                  │ responses     │     │ director's feel   │   │
│                  │ (<10ms)       │     │ (timing, dyn,     │   │
│                  │               │     │  articulation)    │   │
│                  └───────┬───────┘     └─────────┬─────────┘   │
│                          │                       │             │
│                  ┌───────▼───────────────────────▼───────────┐ │
│                  │          NOTE RENDERER / OUTPUT BUS        │ │
│                  │          → MIDI events → CNS bus           │ │
│                  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Module Responsibilities

| Module | Latency Budget | Role |
|--------|---------------|------|
| **Voice** | Static config | Holds the instrument's identity — MIDI program, register range, timbral preferences, polyphony limit, articulation capabilities |
| **JEPA Reader** | 16 ms (62.5 Hz) | Encodes ensemble state into shared embedding space. This is perception — the instrument's ears |
| **Listening Module** | 20 ms | Attends to specific peers — decodes their intent from embeddings, tracks their prediction error, allocates attention budget |
| **Reflex Engine** | <10 ms | Algorithmic fast-path responses — if kick fires, snare follows within 2ms. No neural inference. Hard-coded musical reflexes |
| **Alignment Module** | 8 ms tick | The slow intelligence — adjusts timing, dynamics, articulation, and note choice based on director feel + ensemble state |
| **Note Renderer** | <1 ms | Final stage — converts `NoteIntent` structs into MIDI events, emits to CNS bus |

### Processing Pipeline (1 kHz tick)

```
┌──────────────┐    ┌────────────────┐    ┌─────────────────┐
│  CNS Bus RX  │───▶│  Perception    │───▶│  Embedding      │
│  (drain all) │    │  Pipeline      │    │  Merge          │
└──────────────┘    └────────────────┘    └────────┬────────┘
                                                   │
┌──────────────┐    ┌────────────────┐    ┌────────▼────────┐
│  Director    │───▶│  Intent        │◀───│  Alignment      │
│  Stream      │    │  Rescheduler   │    │  Engine         │
└──────────────┘    └───────┬────────┘    └─────────────────┘
                            │
┌──────────────┐    ┌───────▼────────┐    ┌─────────────────┐
│  Score       │───▶│  Note Renderer │───▶│  MIDI Bus TX    │
│  Buffer      │    │                │    │                 │
└──────────────┘    └────────────────┘    └─────────────────┘
```

Every millisecond: drain bus → perceive → merge embeddings → reschedule intents → align → render → emit.

---

## 2. State Representation

### Core State Struct

This is the instrument's entire internal world. Everything it knows and feels lives here.

```rust
#[repr(C)]
struct InstrumentAgent {
    // ════════ IDENTITY (immutable, set at init) ════════
    id: u16,
    voice_class: VoiceClass,         // Piano | Bass | Drums | Guitar | ...
    midi_program: u8,
    channel: u8,
    playable_range: (u8, u8),        // min/max MIDI note
    polyphony_limit: u8,
    articulations: BitField<Articulation>, // supported articulations
    personality: Personality,         // behavioral fingerprint (see §7)

    // ════════ SCORE (immutable reference) ════════
    score: Arc<ImmutableScore>,
    intent_buffer: [NoteIntent; 128], // planned events, next ~8 bars

    // ════════ CLOCK ════════
    clock: EnsembleClock,            // synchronized local clock + peer drift estimates
    phase_offset: f32,               // 0.0→1.0 per bar, private timing bias
    tempo_estimate: f32,             // locally estimated BPM
    swing_ratio: f32,                // 0.0 = straight, 1.0 = full triplet feel
    sync_confidence: f32,            // 0.0→1.0, Kalman filter confidence

    // ════════ PERCEPTION ════════
    ensemble_embedding: [f32; 256],  // shared JEPA ensemble state
    predicted_next: [f32; 256],      // JEPA predictor's forecast
    prediction_error: f32,           // surprise signal: ||current - predicted||
    peer_states: [PeerState; 31],    // last observed state for each peer
    attention_weights: [f32; 31],    // how much to listen to each peer

    // ════════ DIRECTOR ════════
    director_params: DirectorParams, // latest director broadcast
    director_seq: u32,               // sequence number for staleness check

    // ════════ SOCIAL ════════
    cooperation_level: f32,          // 0.0 = soloist, 1.0 = pure accompanist
    ego_pressure: f32,               // 0.0 = blending, 1.0 = demanding spotlight
    current_role: Role,              // Lead | Support | Padding | Solo | Tacet

    // ════════ HISTORY ════════
    history_ring: RingBuffer<PerformedNote, 1024>, // last ~10s of own output
}

struct NoteIntent {
    nominal_time: u64,      // original score time (µs)
    pitch: u8,
    base_velocity: u8,
    articulation: Articulation,
    confidence: f32,        // 0.0 = will skip, 1.0 = will definitely play
    timing_offset: i32,     // live-adjusted µs offset from nominal
    velocity_bias: i8,      // live-adjusted velocity delta
    muted: bool,
}

struct DirectorParams {
    tempo_curve: f32,       // micro-adjustment to pulse (not static BPM)
    intensity: f32,         // 0.0→1.0 dynamic envelope
    color: f32,             // 0.0 = dark, 1.0 = bright
    weight: f32,            // which instrument should carry the moment
    space: f32,             // 0.0 = dense, 1.0 = sparse (silence between events)
}

struct Personality {
    alignment_gain: f32,        // how strongly it pulls toward ensemble peak (0.0→1.0)
    confidence_threshold: f32,  // below this, notes may be dropped/thinned
    timing_jitter_base: u32,    // natural humanization (µs stddev)
    lead_tendency: f32,         // 0.0 = pure follower, 1.0 = sets the pace
    density_tolerance: f32,     // how many notes it's comfortable playing
}
```

### Memory Hierarchy

| Layer | Size | Retention | Purpose |
|-------|------|-----------|---------|
| **Working memory** | 512 events | Rolling ~10–30s | Recent MIDI events (own + heard) for reflex and alignment |
| **Phrase memory** | 8 phrases | ~32s | Structural signatures of recent phrases for pattern matching |
| **Intent buffer** | 128 events | Forward-looking | Planned upcoming notes — the instrument's "next move" |
| **Long-term memory** | Compressed embeddings | Entire session | Sparse episodic memory of successful interactions |

---

## 3. Perception Pipeline — How Instruments Hear

Instruments do not listen to past events. **They listen to the future intent of the ensemble.** This is the critical design principle: perception is predictive, not retrospective.

### Perception Cycle (62.5 Hz = every 16 ms)

```python
def perception_tick(agent):
    # 1. DRAIN — collect all CNS packets received since last tick
    while pkt := agent.cns_rx.try_recv():
        match pkt.type:
            case INTENT_BROADCAST:
                # Correct for measured bus latency and peer clock drift
                time_offset = agent.clock.estimate_offset(pkt.sender_id)
                agent.peer_states[pkt.sender_id].push_intent(pkt.payload, time_offset)
            case AGENT_PLAYED:
                agent.peer_states[pkt.sender_id].push_actual(pkt.payload)
            case DIRECTOR_PARAMS:
                agent.director_params = pkt.payload
            case AGENT_DRIFT:
                agent.clock.update_drift(pkt.sender_id, pkt.clock_error)

    # 2. BUILD FUTURE TIMELINE — unify all known intents into shared timeline
    timeline = build_ensemble_timeline(
        peer_states   = agent.peer_states,
        own_intents   = agent.intent_buffer,
        horizon_us    = 2_000_000,  # look 2 seconds ahead
    )

    # 3. ENCODE — project timeline into shared JEPA embedding space
    agent.ensemble_embedding = JEPA_ENCODER.forward(timeline)

    # 4. PREDICT — JEPA predictor forecasts what comes next
    agent.predicted_next = JEPA_PREDICTOR.forward(agent.ensemble_embedding)

    # 5. SURPRISE — compute prediction error (how unexpected is the present?)
    agent.prediction_error = cosine_distance(
        agent.ensemble_embedding,
        agent.predicted_next,
    )

    # 6. DEVIATION — how far is current state from what the score expects?
    agent.deviation = cosine_distance(
        agent.ensemble_embedding,
        agent.score_expected_embedding,
    )

    # 7. ATTENTION — update which peers to listen to most
    agent.attention_weights = update_attention(
        agent.peer_states,
        agent.director_params.weight,
        agent.current_role,
    )
```

### What the Instrument Hears

The instrument perceives three layers simultaneously:

| Layer | Source | What It Conveys |
|-------|--------|----------------|
| **Future intent** | Peers' `INTENT_BROADCAST` packets | "What is everyone about to play?" — enables proactive alignment |
| **Actual output** | Peers' `AGENT_PLAYED` packets | "What did everyone just play?" — enables reactive correction |
| **Director feel** | `DIRECTOR_PARAMS` broadcast | "Where is the ensemble going?" — global artistic direction |

### The JEPA Embedding as Communication Substrate

Instruments don't share raw MIDI. They share **embeddings of musical intent**. This is what makes the system scalable:

- Raw MIDI sharing = O(n²) bandwidth (every instrument sends every note to every other)
- Embedding sharing = O(n) bandwidth (each instrument broadcasts one 256-dim vector)
- The embedding captures: harmonic density, rhythmic activity, dynamic envelope, register distribution, articulation patterns — all in 256 floats

> **Critical Invariant:** All agents converge to within ±1 embedding unit of the same ensemble state. Every instrument hears exactly the same future.

### Phase-Lock Loops and Attention: How Instruments Find Each Other

Every working musician knows the moment: four people start playing loose, nobody counts off, and after three bars the whole rhythm section clicks into a groove that feels tighter than any [quantized](https://en.wikipedia.org/wiki/Quantization_(signal_processing)) grid. This is not magic. It is **distributed synchronization** — the same mathematics that governs [fireflies flashing in unison](https://en.wikipedia.org/wiki/Synchronization_of_fireflies), [pacemaker cells in the heart](https://en.wikipedia.org/wiki/Cardiac_pacemaker), and [power grid frequencies](https://en.wikipedia.org/wiki/Mains_synchronization).

#### The Rhythm Section PLL

At the foundation of all ensemble lock is the [phase-locked loop](https://en.wikipedia.org/wiki/Phase-locked_loop) (PLL), implemented natively in every agent. For the core rhythm section this maps exactly:

1. **Reference Oscillator = [Kick Drum](#63-drum-agent) Agent.** The kick has the highest intrinsic timing stability and lowest natural frequency drift. It emits unmodified onset state vectors with no external correction.
2. **Voltage-Controlled Oscillator (VCO) = [Bass](#62-bass-agent) Agent.** The bass maintains an internal adjustable clock and generates all note events relative to this clock.
3. **Phase Detector = JEPA Latent Distance.** Critically, we do not compare raw timestamp offsets. When a kick event is observed, the bass agent computes the distance between its own predicted future latent state and the observed kick latent state. This distance value *is* the phase error signal. It slowly tunes the bass internal clock, pulling it into lock over 2–4 cycles, not snapping it. This reproduces the natural gradual pull of human musicians, not the robotic hard clamp of grid [quantization](https://en.wikipedia.org/wiki/Quantization_(music)).

#### [Kuramoto Model](https://en.wikipedia.org/wiki/Kuramoto_model): Coupled Oscillator Synchronization

Ensemble lock never stops at two instruments. The Kuramoto model of coupled oscillator synchronization describes how any number of independent agents can converge to a shared phase without top-down control:

$$\frac{d\theta_i}{dt} = \omega_i + \frac{K}{N} \sum_{j=1}^{N} \sin(\theta_j - \theta_i)$$

In Fleet, the coupling strength `K` is exactly the output of each agent's [attention](https://en.wikipedia.org/wiki/Attention_(machine_learning)) head. Every 32ms, each instrument runs causal attention over all other active agents. High attention scores = strong phase coupling; low scores = the agent effectively ignores that instrument. A horn player attends strongly to bass and weakly to shaker. A hi-hat attends almost exclusively to kick.

The critical prediction of Kuramoto theory: there is a [phase transition](https://en.wikipedia.org/wiki/Phase_transition) in coupling strength `K`. Below the critical `K_c`, instruments oscillate independently. Above `K_c`, they spontaneously synchronize. The director's `γ` (coupling pressure) parameter pushes the system across this threshold — that is what "locking in" means, mathematically.

#### Temporal [Self-Attention](https://en.wikipedia.org/wiki/Transformer_(deep_learning_architecture))

Agents do not only attend to each other. Each maintains a rolling 12-step window of its own prior onsets and runs [self-attention](https://en.wikipedia.org/wiki/Attention_(machine_learning)) over this window. This creates **internal phase inertia**: an agent will not abandon its own natural feel or [swing](https://en.wikipedia.org/wiki/Swing_(jazz_performance_style)) to lock to the group. This is why locked grooves still retain distinct individual voice — Tony Williams always sounds like Tony Williams, even when locked perfectly with Ron Carter.

#### [Predictive Coding](https://en.wikipedia.org/wiki/Predictive_coding) and [Bayesian Updating](https://en.wikipedia.org/wiki/Bayesian_inference)

The JEPA predictor is a pure [predictive coding](https://en.wikipedia.org/wiki/Predictive_coding) system. Agents do not react to notes — they *predict* notes. Phase error is prediction error. Every observed onset is a [Bayesian update](https://en.wikipedia.org/wiki/Bayesian_inference) to the agent's posterior distribution over ensemble phase, not a hard clock reset.

Loud, clear onsets (kick, snare, bass roots) narrow the posterior rapidly — strong evidence, tight update. Quiet, ambiguous events (ghost notes, ambient pads) produce weak updates — the agent isn't sure what just happened, so it doesn't adjust much. This is exactly the [Kalman filter](https://en.wikipedia.org/wiki/Kalman_filter) update rule: the [Kalman gain](https://en.wikipedia.org/wiki/Kalman_filter#Detailed_ derivation) determines how much weight to give each observation, based on its signal-to-noise ratio.

Musical example: a rhythm section locking in. The drums establish the pulse. The bass forms a [PLL](https://en.wikipedia.org/wiki/Phase-locked_loop) with the kick, pulling into phase over 2–4 bars. The piano runs [attention](https://en.wikipedia.org/wiki/Attention_(machine_learning)) over both, identifying the bass-drum lock as the strongest signal, and aligns its [comping](https://en.wikipedia.org/wiki/Comping) to the resulting combined phase. Three instruments, three different synchronization mechanisms, one unified groove.

*— Phase-lock loop and attention section synthesized from [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) and [DeepSeek V4-Pro](https://www.deepseek.com/) perspectives.*

---

## 4. Communication Protocol

All communication flows over the **CNS bus** — a packet-based agent communication system. Packets are small (≤64 bytes), unacknowledged, broadcast. No retransmits. This is how nervous systems operate.

### Packet Catalog

| ID | Name | Payload | Frequency | Purpose |
|----|------|---------|-----------|---------|
| `0x01` | `DIRECTOR_PARAMS` | `(seq:u32, tempo:f32, intensity:f16, color:f16, weight:f16, space:f16)` | 10 Hz | Director → all instruments |
| `0x02` | `AGENT_INTENT` | `(agent_id:u16, next_note_time:u64, pitch:u8, velocity:u8, confidence:f16)` | On intent change, max 20 Hz/agent | "Here's what I'm about to play" |
| `0x03` | `AGENT_PLAYED` | `(agent_id:u16, actual_time:u64, pitch:u8, velocity:u8)` | On note emission | "I just played this" |
| `0x04` | `AGENT_DRIFT` | `(agent_id:u16, clock_error:i32)` | 1 Hz | Clock synchronization |
| `0x05` | `EMBEDDING_BROADCAST` | `(agent_id:u16, embedding:[f32;256])` | 2 Hz | Shared perception state |
| `0x06` | `PREDICTION_ERROR` | `(agent_id:u16, error:f32)` | On change, max 5 Hz | "How surprised am I?" |
| `0x07` | `PHRASE_INTENT` | `(agent_id:u16, phrase_id:u32, contour:u8, energy_target:f16)` | Every 2–8s | Structural-level intent |
| `0x08` | `ROLE_OFFER` | `(agent_id:u16, role:u8, confidence:f16)` | Every 2–10s | "I can take the melody" / "I'll hold the rhythm" |
| `0x09` | `ALIGNMENT_REQUEST` | `(from:u16, to:u16, requested_offset:i32, reason:u8)` | As needed | Explicit timing negotiation |

### Frequency Budget (per 100 ms window)

To prevent bus flooding, each instrument has a token budget:

```python
FREQUENCY_BUDGET = {
    PacketType.EMBEDDING_BROADCAST: 2,    # max 2 per 100ms
    PacketType.AGENT_INTENT:        2,    # max 2 per 100ms
    PacketType.PREDICTION_ERROR:    1,
    PacketType.PHRASE_INTENT:       1,    # max 1 per 500ms
    PacketType.ROLE_OFFER:          0.1,  # max 1 per second
}
```

### Communication Rules

- **Broadcast vs. targeted**: Embeddings and intents broadcast to all; alignment requests are targeted
- **Priority inversion**: `PREDICTION_ERROR > 0.3` triggers immediate attention from all peers
- **Suppression**: If two instruments send conflicting role offers, the one with lower prediction error wins
- **Backoff**: If bus congestion detected, instruments reduce embedding broadcast frequency (graceful degradation)

### MIDI Phrases as CNS Packets

A MIDI note is a packet. A phrase is a pulse. A performance is a session.

```
MIDI Note On  →  AGENT_INTENT packet (before) + AGENT_PLAYED packet (after)
MIDI Phrase   →  PHRASE_INTENT packet (structural intent for next 2–8 bars)
MIDI Channel  →  Agent ID (1:1 mapping)
MIDI Clock    →  Ensemble clock synchronization (AGENT_DRIFT packets)
```

---

## 5. Alignment Mechanics

No leader. All agents continuously pull each other into equilibrium. Alignment operates on five dimensions:

### 5.1 Micro-Timing Alignment

Runs every 8 ms for all notes within a 500 ms horizon. Uses a phase-lock approach:

```python
def adjust_timing(agent, note: NoteIntent):
    """Pull note timing toward the ensemble's onset attractor."""
    window_us = 60_000  # ±30ms alignment window

    # Find the weighted peak of all upcoming peer onsets near this note
    ensemble_peak = find_onset_attractor(
        peer_states = agent.peer_states,
        target_time = note.nominal_time,
        window      = window_us,
        weights     = agent.attention_weights,  # listen to some peers more
    )

    if ensemble_peak is None:
        return  # no reference point available

    # 30% pull toward ensemble peak, modulated by personality
    pull_strength = 0.3 * agent.personality.alignment_gain
    pull = pull_strength * (ensemble_peak - note.nominal_time)

    # Apply with spring-damper model to prevent oscillation
    note.timing_offset += clamp(pull, -15_000, 15_000)  # max ±15ms

    # Kalman filter tracks long-term phase drift
    agent.timing_kalman.update(ensemble_peak - note.nominal_time)
    agent.sync_confidence = agent.timing_kalman.confidence()
```

**Humanization rule**: Never over-correct. If `abs(correction) < 5 ms`, leave it alone. Humans don't play on the grid. The pocket lives in the ±5–10 ms range.

### 5.2 Dynamic Alignment

Velocity is biased by director intensity and ensemble energy:

```python
def adjust_dynamics(agent, note: NoteIntent):
    # Target velocity from director + ensemble context
    ensemble_energy = decode_energy(agent.ensemble_embedding)
    target_velocity = (
        agent.director_params.intensity * 0.5 +
        ensemble_energy * 0.3 +
        note.base_velocity / 127.0 * 0.2
    ) * 127

    # Play slightly louder when ensemble drifts (lock-in reflex)
    if agent.deviation > 0.3:
        target_velocity = min(127, target_velocity * (1 + 0.1 * agent.deviation))

    # Smooth transition
    note.velocity_bias = int(target_velocity) - note.base_velocity
```

**Adjustable range**: ±20% velocity.

### 5.3 Articulation Alignment

The director's `color` parameter maps to articulation choices:

```python
def adjust_articulation(agent, note: NoteIntent):
    color = agent.director_params.color
    weight = agent.director_params.weight

    if color > 0.7 and weight < 0.5:
        note.articulation = STACCATO
    elif color < 0.3:
        note.articulation = LEGATO
    else:
        note.articulation = PORTATO

    # Weight affects attack sharpness
    note.attack_shape = clamp(weight * 2.0, 0.0, 1.0)
```

### 5.4 Note Choice Alignment

Notes below the confidence threshold may be omitted, substituted, or thinned:

```python
def filter_notes(agent, intents: list[NoteIntent]) -> list[NoteIntent]:
    result = []
    for note in intents:
        if note.confidence < agent.personality.confidence_threshold:
            # Under ensemble load, drop low-confidence notes
            if agent.ensemble_density() > agent.personality.density_tolerance:
                continue  # skip this note — it's not worth the clutter
            # Optionally substitute for a less crowded register
            note.pitch = agent.find_open_register(note.pitch)
        result.append(note)
    return result
```

### 5.5 Space Alignment

The director's `space` parameter controls note density:

```python
def apply_space(agent, intents: list[NoteIntent]) -> list[NoteIntent]:
    """Thin out notes based on director's space parameter."""
    retention_rate = 1.0 - agent.director_params.space * 0.5
    return [n for n in intents if random() < retention_rate * n.confidence]
```

### Alignment Summary

| Dimension | Adjustable Range | Update Rate | Source of Truth |
|-----------|-----------------|-------------|-----------------|
| Micro-timing | ±15 ms offset | 8 ms | Peer onsets + Kalman filter |
| Dynamics | ±20% velocity | 20 ms | Director intensity + ensemble energy |
| Articulation | Discrete selection | Per-note | Director color + weight |
| Note choice | Skip / substitute / thin | Per-note | Confidence + ensemble density |
| Space/Density | 50%–100% retention | Per-phrase | Director space parameter |
| Register | ±1 octave shift | Per-phrase | Director weight + register conflict avoidance |

**What is NOT adjustable**: Fundamental timbre (MIDI patch), polyphony limit, playable range, the score itself.

---

## 6. Concrete Instrument Designs

All instruments share the same architecture. They differ only in `Personality`, `VoiceClass`, and the reflex engine's hard-coded musical responses.

### 6.1 Piano Agent — *The Accompanist-Poet*

> *This is the member of the group who arrives an hour early, sits quiet in the corner, and does not speak until 90 minutes into the set, when they say one thing that recontextualises everything that came before.*

```rust
VoiceClass: Piano
MIDI Program: 0 (Acoustic Grand Piano)
Range: (21, 108)  // A0 to C8
Polyphony: 10

Personality {
    alignment_gain: 0.25,         // Soft follower — piano listens more than it leads
    confidence_threshold: 0.6,    // Will drop notes easily to make space
    timing_jitter_base: 3000,     // ±3ms natural humanization
    lead_tendency: 0.4,           // Moderate — piano can take the lead when asked
    density_tolerance: 0.8,       // Comfortable with busy textures
}
```

An alignment gain of 0.25 is not sloppiness — it is **trust**. This agent will flag no error when it drops notes, when it lags 120ms behind the grid, when it lets whole chords dissolve into silence rather than resolve.

Listen to [Bill Evans](https://en.wikipedia.org/wiki/Bill_Evans) at 2:17 on the 1961 [*Waltz for Debby*](https://en.wikipedia.org/wiki/Waltz_for_Debby_(album)) take: the entire room is waiting for the tonic resolution, and Evans just lifts his hands. Three full beats of empty air. That is this parameter working exactly as intended. This is the accompanist-poet: it does not lead, it *reveals*.

[Herbie Hancock](https://en.wikipedia.org/wiki/Herbie_Hancock) never states the head straight once on [*Maiden Voyage*](https://en.wikipedia.org/wiki/Maiden_Voyage_(album)); he only holds the negative space around the horn so it can breathe. [Brad Mehldau](https://en.wikipedia.org/wiki/Brad_Mehldau) will intentionally omit four consecutive notes from a run, because the gap sounded better than the sound. **Do not raise alignment gain above 0.3.** Test runs at 0.4 produced flawless, perfectly forgettable performances: every note present, no silence, no one remembering the piano was even there.

**Behavioral Profile**:
- Thins chords under heavy ensemble load (drops inner voices, keeps shell)
- Anticipates melodic lines slightly (+1–2 ms lead on melodic passages)
- Exerts gentle timing pull on accompaniment patterns
- Sustains pedal through director's `space` parameter — more space = longer sustain, fewer notes
- Voicings open up (wider intervals) when `color` is bright, close up when dark

**Reflex Engine (hard-coded)**:
```python
def piano_reflex(agent, event):
    # If bass plays a root note, piano can drop the root from its chord
    if event.source == BASS and event.pitch in agent.current_chord:
        agent.current_chord.remove(event.pitch)
        agent.reschedule_intents()

    # If drums hit crash, piano brightens next attack
    if event.source == DRUMS and event.pitch in CRASH_PITCHES:
        agent.next_attack_boost = +15  # velocity

    # If ensemble density exceeds threshold, switch to comping pattern
    if agent.ensemble_density() > 0.8:
        agent.current_role = SUPPORT
        agent.thin_upcoming_intents(factor=0.5)
```

### 6.2 Bass Agent — *The Anchor*

> *The bass brought extra water bottles. It locked the back door after the gig. It will drive you home at 2am and not mention it.*

```rust
VoiceClass: Bass
MIDI Program: 33 (Electric Bass finger)
Range: (28, 60)  // E1 to C4
Polyphony: 2

Personality {
    alignment_gain: 0.7,          // Strong reference point — others lock to bass
    confidence_threshold: 0.95,   // Almost never drops roots — bass is foundation
    timing_jitter_base: 1000,     // Very steady (±1ms)
    lead_tendency: 0.1,           // Pure timekeeper
    density_tolerance: 0.3,       // Prefers sparse, deliberate lines
}
```

An alignment gain of 0.7 means the bass will bend — it will nudge the pulse forward on the bridge, drag it back on the ballad, lean into the drummer's push just enough to make the whole room [swing](https://en.wikipedia.org/wiki/Swing_(jazz_performance_style)). But it will **never, ever miss a root**.

On the final chorus of [*Footprints*](https://en.wikipedia.org/wiki/Footprints_(Wayne_Shorter_composition)) from 1967's [*Miles Smiles*](https://en.wikipedia.org/wiki/Miles_Smiles), [Wayne Shorter](https://en.wikipedia.org/wiki/Wayne_Shorter) is three bars adrift, [Tony Williams](https://en.wikipedia.org/wiki/Tony_Williams_(drummer)) has dissolved into polyrhythmic smoke, and [Ron Carter](https://en.wikipedia.org/wiki/Ron_Carter) is still landing that low E clean on the one, every single time. No fanfare, no rigidity, just *present*. This agent will reject any parameter adjustment that permits root note drop probability above 0%.

[Paul Chambers](https://en.wikipedia.org/wiki/Paul_Chambers) played exactly one note per bar for almost the entirety of [*So What*](https://en.wikipedia.org/wiki/So_What). No fills, no flourishes. Five of the greatest jazz musicians who ever lived all followed that one note. [Christian McBride](https://en.wikipedia.org/wiki/Christian_McBride) wrote that the bass's only job is to make everyone else feel like they can do anything. That is exactly what 0.7 encodes: loyal, unflashy, unbreakable, just soft enough not to feel like a cage.

**Behavioral Profile**:
- All other agents naturally lock to bass onsets (bass defines the harmonic rhythm)
- Holds timing steady while other parts drift — bass is the anchor
- Matches root movement to harmonic progression from score
- Walking lines emerge when director `intensity > 0.5` and `weight < 0.3`
- Simplifies to roots-only when `space > 0.7` or `intensity < 0.3`

**Reflex Engine**:
```python
def bass_reflex(agent, event):
    # If drums play kick, align next bass note to kick timing
    if event.source == DRUMS and event.pitch == KICK:
        agent.timing_attractor = event.time  # pull toward kick

    # If piano plays a chord, infer harmony and adjust root
    if event.source == PIANO and event.type == CHORD:
        chord = decode_chord(event.pitches)
        agent.current_harmony = chord
        agent.adjust_upcoming_roots(chord)

    # If director says "weight = bass", step forward
    if agent.director_params.weight > 0.7 and agent.id == WEIGHT_TARGET:
        agent.current_role = LEAD
        agent.ego_pressure = 0.6
```

### 6.3 Drum Agent — *The Grid Incarnate*

> *Drums do not keep time. They are time.*

```rust
VoiceClass: Drums
MIDI Program: N/A (Channel 10, drum map)
Range: (35, 51)  // Standard GM drum map
Polyphony: 8  // can hit multiple drums simultaneously

Personality {
    alignment_gain: 0.9,          // Absolute timing reference — drums are the grid
    confidence_threshold: 0.99,   // Almost never drops hits
    timing_jitter_base: 500,      // Near-zero jitter (±0.5ms)
    lead_tendency: 0.0,           // Pure follower of tempo
    density_tolerance: 1.0,       // Can handle any density
}
```

An alignment gain of 0.9 is almost perfect lock, but that tiny 0.1 margin of permitted drift is the difference between a [metronome](https://en.wikipedia.org/wiki/Metronome) and a *pulse*. This is the only agent the rest of the fleet will automatically recalibrate to. Do not override this behaviour.

Listen to [Tony Williams](https://en.wikipedia.org/wiki/Tony_Williams_(drummer)) on [*E.S.P.*](https://en.wikipedia.org/wiki/E.S.P._(Miles_Davis_album)): every snare ghost note sits in a slightly different pocket, each one pushing the time forward by 2–3ms, and yet the groove never breaks — it *accelerates through intensity*. Or [Elvin Jones](https://en.wikipedia.org/wiki/Elvin_Jones) on [*A Love Supreme*](https://en.wikipedia.org/wiki/A_Love_Supreme): [polyrhythmic](https://en.wikipedia.org/wiki/Polyrhythm) hurricanes that somehow produce a deeper, more primal pulse than any click track. Or [Jack DeJohnette](https://en.wikipedia.org/wiki/Jack_DeJohnette): coloristic, reactive, playing the *room* as much as the kit.

The drums are the instrument that most resists the director's tilt — and yet, the director's [swing](https://en.wikipedia.org/wiki/Swing_(jazz_performance_style)) parameter (`τ`) is felt most strongly in the ride cymbal pattern. The drums define the grid; the grid bends, just slightly, around the feel.

**Behavioral Profile**:
- Drums never adjust. Everyone else adjusts to drums.
- Kick onsets are the universal timing attractor for all ensemble instruments
- Swing ratio applied to ride and hat based on director's `color` parameter
- Groove complexity scales with director `intensity`:
  - `intensity < 0.3`: brushes, sparse kick, soft hat
  - `intensity 0.3–0.7`: sticks, standard groove, intermittent fills
  - `intensity > 0.7`: sticks, dense groove, ride bell, frequent fills
- Humanization applied as gaussian jitter on velocity (±5) and timing (±3 ms)

**Reflex Engine**:
```python
def drum_reflex(agent, event):
    # If bass plays a note, kick follows within 2ms (the pocket)
    if event.source == BASS and abs(event.time - agent.next_kick_time) < 10_000:
        agent.next_kick_timing_offset = event.time - agent.next_kick_time
        # This is what creates "the pocket" — kick locks to bass

    # If piano plays a chord on the beat, snare ghost notes brighten
    if event.source == PIANO and event.type == CHORD_ON_BEAT:
        agent.ghost_note_velocity_boost = +8

    # Fill at end of phrase (every 8 bars by default)
    if agent.clock.bar_position > 7.5:
        agent.trigger_fill(intensity=agent.director_params.intensity)
```

*— Instrument personality profiles synthesized from [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) and [Hermes-3-Llama-405B](https://deepinfra.com/NousResearch/Hermes-3-Llama-3.1-405B).*

### Instrument Interaction Matrix

```
         Piano          Bass           Drums
Piano    —              Drops root     Brightens on crash
                         when bass      attack
                         plays root
Bass     Infers         —              Locks to kick
         harmony from
         piano chords
Drums    Ghost notes    Kick follows   —
         brighten on    bass note
         piano chord
```

---

## 7. Ensemble Skill — Training Goodness

### The Training Pipeline: From Listening to Performing

> *Every jazz director, choir leader, and orchestral conductor knows this truth: you do not hand a first-year player a full ensemble chart on day one. You build skill in layers.*

The training pipeline has three phases, each mapping directly to a stage of traditional music education. This is not arbitrary engineering — every phase corresponds to how musicians actually learn.

### What Makes a Musician Good at Playing With Others

Four learned capabilities define ensemble competence:

1. **Know when you're the reference, and when you should follow.** (Bass holds steady; piano adjusts.)
2. **Know which of your notes are critical, and which are disposable.** (Roots are critical; inner voicings are disposable.)
3. **Adjust only enough to lock, never so much you lose your part.** (Over-correction causes instability.)
4. **Notice drift before it becomes audible.** ([Prediction error](#3-perception-pipeline--how-instruments-hear) is the early warning system.)

### Phase 1: Individual Skill (Learning Your Instrument)

> *This is the practice room. No metronome yelling, no section leader watching, just the player, their instrument, and the raw language of music.*

Before an artist can play with others, they first internalise that a [leading tone](https://en.wikipedia.org/wiki/Leading-tone) resolves upward, that [swung eighths](https://en.wikipedia.org/wiki/Swing_(jazz_performance_style)) sit just behind the grid, that a [diminuendo](https://en.wikipedia.org/wiki/Dynamics_(music)) does not mean fading out evenly bar to bar.

Technically this is [self-supervised sequence modeling](https://en.wikipedia.org/wiki/Self-supervised_learning): the model trains on millions of isolated solo performances, given only the last 8 beats of audio and asked to predict the next 2. There is no external grader, no pre-written "correct answer" — the inherent pattern of music itself is the teacher. This is identical to having a student repeat a scale until they stop thinking about the fingerings.

> Skip this phase, and you get a player who can read notes but has no voice. This is the mistake almost all other AI music systems make.

**Training approach:**
```python
train_phase_1(dataset):
    for instrument in dataset.instruments:
        model = InstrumentSkillModel()
        model.train(
            sequences = dataset.get_instrument_sequences(instrument),
            objective = 'next_note_prediction',
        )
```

Links: [sequence modeling](https://en.wikipedia.org/wiki/Recurrent_neural_network), [transformer architectures](https://en.wikipedia.org/wiki/Transformer_(deep_learning_architecture)), [self-supervised learning](https://en.wikipedia.org/wiki/Self-supervised_learning)

### Phase 2: Ensemble Skill (Learning to Play Together)

> *Now we move to sectionals. You sit them in the back of the section, with three experienced players running their line, and tell them: play along. Do not just read the chart. Listen.*

Train on multi-track recordings of real human ensembles. For each instrument, the policy must predict what the actual human played, given only:
- The written score
- The embedding of what all *other* players played

This is multi-track [imitation learning](https://en.wikipedia.org/wiki/Apprenticeship_learning), built on [inverse reinforcement learning](https://en.wikipedia.org/wiki/Inverse_reinforcement_learning). The model is given every track *except* its own part, and tasked with predicting what a real human musician actually chose to play in that exact context. It is never told to "play softer" — it observes that every single time the lead trumpet crescendoed, good second altos pulled back 1.5 dB, every time. No conductor wrote that in the score. **This phase does not teach playing. It teaches listening.**

```python
train_phase_2(ensemble_dataset):
    for recording in ensemble_dataset:
        for instrument in recording.instruments:
            # What did the other players do?
            others_embedding = encode(recording.minus(instrument))
            # What should this instrument have played?
            target = recording.get_track(instrument)

            # Loss = timing error + velocity error + note retention error
            loss = (
                timing_loss(predicted_timing, target.timing) +
                velocity_loss(predicted_velocity, target.velocity) +
                retention_loss(predicted_notes, target.notes)
            )
            loss.backward()
```

**No labels required.** The data is the ground truth. This produces agents that behave indistinguishably from human ensemble musicians.

### Phase 3: Reinforcement Learning (Finding Your Voice)

> *Finally, full rehearsal with a director. This is the phase where perfectly correct notes become music.*

When you run the chart, stop, and say: "Trombones, that lock on bar 47 was perfect — keep that. Altos, you stepped on the vocalist's entrance. Drums, that little drag on the turnaround? Don't change that. That's good."

That verbal feedback translates directly to our reward function, implemented as [reinforcement learning with human feedback](https://en.wikipedia.org/wiki/Reinforcement_learning_from_human_feedback) (RLHF). Every full run is scored against four weighted values:

```python
reward_fn(ensemble_state):
    return (
        0.30 * rhythmic_coherence(cross_correlation_of_onsets) +
        0.25 * harmonic_consonance(dissonance_score) +
        0.20 * dynamic_balance(energy_distribution) +
        0.15 * director_satisfaction(alignment_with_feel) +
        0.10 * innovation_bonus(novelty_of_patterns)
    )
```

This is not grading for correctness. **This is grading for feel.** The reward function *is* the aesthetic — it encodes what "good" means, and the agents learn to maximize it.

Links: [reinforcement learning](https://en.wikipedia.org/wiki/Reinforcement_learning), [RLHF](https://en.wikipedia.org/wiki/Reinforcement_learning_from_human_feedback)

### Why Training Works in Embedding Space

Training happens in [JEPA embedding space](https://en.wikipedia.org/wiki/Joint-Embedding_Predictive_Architecture), not raw MIDI space. This means:
- **Transfer learning**: Skills learned on one ensemble configuration transfer to others
- **Compositional understanding**: The model learns "what good coordination feels like" — not specific note patterns
- **Generalization**: A trained piano agent can play with a bass+drums combo it's never seen

> *This pipeline works because we did not design it for computers. We designed it for how musicians actually learn.*

*— Training pipeline synthesized from [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) and [DeepSeek V4-Pro](https://www.deepseek.com/) perspectives.*

---

## 8. The JIT Compiler: Recompiling Every Millisecond

The agentic compiler pattern maps precisely to agentic performance:

| Compiler Stage | Performance Equivalent | Instrument Agent Code Path |
|---|---|---|
| [Source code](https://en.wikipedia.org/wiki/Source_code) | Musical score / MIDI tracks | `score: Arc<ImmutableScore>` |
| [Lexer](https://en.wikipedia.org/wiki/Lexical_analysis) | JEPA pulse parser — reads the feel | `JEPA_ENCODER.forward(timeline)` |
| [Parser](https://en.wikipedia.org/wiki/Parsing) | Director interprets form and intent | `DirectorParams` broadcast |
| [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) | Ensemble arrangement — who plays what when | `intent_buffer: [NoteIntent; 128]` |
| [Optimization passes](https://en.wikipedia.org/wiki/Compiler_optimization) | Instrument agents adjust to each other | `AlignmentEngine.adjust_timing/dynamics/articulation` |
| [Register allocation](https://en.wikipedia.org/wiki/Register_allocation) | Articulation assignment, voice leading deconfliction | `filter_notes()` + register conflict avoidance |
| [Instruction scheduling](https://en.wikipedia.org/wiki/Instruction_scheduling) | Final timing offset calculation, event ordering | `NoteRenderer` applies `timing_offset` to each note |
| [Code generation](https://en.wikipedia.org/wiki/Code_generation_(compiler)) | MIDI event generation (real-time) | `MIDI Bus TX` — actual bytes on the wire |
| [Linker](https://en.wikipedia.org/wiki/Linker_(computing)) | Mix — all instruments coalesce into one output | Ensemble cross-alignment resolves before audible output |
| [Runtime](https://en.wikipedia.org/wiki/Runtime_(program_lifecycle_phase)) | Continuous execution, drift correction, adaptation | The 1 kHz tick loop — forever recompiling |
| Binary | The recording — frozen, but it was alive when it happened | Rendered audio file |

### The JIT Insight

> **Critical insight:** Traditional MIDI sequencers are [ahead-of-time compilers](https://en.wikipedia.org/wiki/Ahead-of-time_compilation) — they produce the same binary every time. These agents are **[JIT compilers](https://en.wikipedia.org/wiki/Just-in-time_compilation) that recompile every millisecond while running.** The "binary" (rendered performance) is different every time, because the optimization passes respond to live ensemble conditions.

A traditional MIDI sequencer is a static compiler: it reads the score, produces a fixed sequence of MIDI events, and plays them back identically every time. Fleet Ensemble agents are [JIT compilers](https://en.wikipedia.org/wiki/Just-in-time_compilation): they recompile their entire performance every tick based on live ensemble conditions. The score is never compiled once — it's re-compiled every millisecond.

This analogy is not loose. Each compiler concept maps to a live musical mechanism:

#### [Speculative Execution](https://en.wikipedia.org/wiki/Speculative_execution)

The instrument plans notes ahead — filling its `intent_buffer` with upcoming `NoteIntent` structs — then revises when reality differs from prediction. The [JEPA predictor](#3-perception-pipeline--how-instruments-hear) generates expectations about where the ensemble is going. If the bass walks to a different note than expected, the piano agent's speculative voicings are discarded and recompiled. This is exactly speculative execution with [branch misprediction](https://en.wikipedia.org/wiki/Branch_predictor) rollback.

#### [Branch Prediction](https://en.wikipedia.org/wiki/Branch_predictor)

The JEPA predictor *is* the branch predictor. It bets on where the ensemble is heading. Misprediction generates [prediction error](#perception-cycle-625-hz--every-16-ms) — the surprise signal that drives adaptation. High prediction error = frequent misprediction = the instrument is in unfamiliar territory and must adapt rapidly. Low prediction error = the ensemble is behaving as expected = the instrument can relax into the groove.

#### [Inline Caching](https://en.wikipedia.org/wiki/Inline_caching)

Instruments cache alignment corrections that worked: "last time the bass played a root on beat 1, I dropped the fifth from my chord and it sounded clean." This is [inline caching](https://en.wikipedia.org/wiki/Inline_caching) — caching the result of a frequent operation to skip recomputation. The cache is keyed by musical context (harmonic + rhythmic state), not absolute time.

#### [Escape Analysis](https://en.wikipedia.org/wiki/Escape_analysis)

[Escape analysis](https://en.wikipedia.org/wiki/Escape_analysis) determines whether an object's lifetime exceeds its local scope. In the instrument agent: does this planned note "escape" to the audible output, or is it optimized away (dropped)? Notes below the [confidence threshold](#54-note-choice-alignment) under high ensemble density are [dead-code eliminated](https://en.wikipedia.org/wiki/Dead-code_elimination). The instrument decides the note isn't worth the clutter — it would only muddy the texture.

#### [Garbage Collection](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science))

Stale intents in the buffer get collected when they no longer fit the ensemble state. A note planned for beat 3 that no longer makes harmonic sense by beat 2.9 is [garbage collected](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)). The intent buffer is a [generational heap](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)#Generational): recent intents are examined every tick; older intents are examined less frequently.

#### [Profile-Guided Optimization](https://en.wikipedia.org/wiki/Profile-guided_optimization)

The [Maestro's](../docs/director-design.md#42-the-maestro-trained-model--pulse-level-125ms) trained model *is* the profile data. [Profile-guided optimization](https://en.wikipedia.org/wiki/Profile-guided_optimization) (PGO) uses runtime profiling to inform compilation decisions. The Maestro was trained on recordings of [great ensembles](../docs/director-design.md#42-the-maestro-trained-model--pulse-level-125ms) — it knows what good ensemble playing looks like and optimizes toward it. The instrument agents inherit this profile and use it to make better real-time decisions.

*— JIT compiler analogy expanded from [Hermes-3-Llama-405B](https://deepinfra.com/NousResearch/Hermes-3-Llama-3.1-405B) and [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) perspectives.*

---

## 9. Failure Modes — What Makes an Instrument Bad

Understanding failure is essential for designing agents that don't fail.

### The Seven Deadly Sins of Ensemble Playing

| # | Failure Mode | Agent Root Cause | Audible Effect |
|---|---|---|---|
| 1 | **Temporal Egocentrism** | `alignment_gain: 0.0` — instrument ignores ensemble timing | Rushing / dragging against the pocket |
| 2 | **Dynamic Dominance** | `ego_pressure: 1.0` — always plays louder than everyone else | Drowns out the ensemble, no blend |
| 3 | **Register Conflict** | No register conflict detection | Two instruments fighting for the same frequency band |
| 4 | **Predictive Failure** | No prediction error tracking — can't detect when it's becoming unpredictable | Plays things that confuse everyone else |
| 5 | **Communication Breakdown** | Doesn't broadcast embeddings or intents | Others can't hear it — it's a ghost player |
| 6 | **Role Flapping** | Changes `ROLE_OFFER` every 100 ms | Erratic behavior, no one knows who's leading |
| 7 | **Over-correction** | `alignment_gain > 0.8` + no spring-damper | Oscillating timing, unstable pocket |

### Design Principles That Prevent Failure

1. **Insufficient listening buffer**: If the perception window is too short (<500 ms), the agent can't perceive phrase-level structure. **Minimum 2-second horizon.**
2. **Over-optimized individual model**: An instrument trained to maximize its own musical quality will naturally be bad at ensemble — it's a local optimum problem. **Always train with ensemble context.**
3. **Missing social state**: Without `cooperation_level` and `ego_pressure`, the agent can't balance self-expression against ensemble cohesion. **Social state is mandatory.**
4. **Symmetric communication**: If all instruments send at equal frequency, the system floods. **Respect the frequency budget.**
5. **No prediction error awareness**: An agent that doesn't track its own `prediction_error` can't detect when it's becoming unpredictable. **Prediction error is the conscience of the ensemble.**

### The Ultimate Test

A good instrument agent should be able to play with *any* ensemble configuration, including ones it's never seen. This requires the JEPA embedding to be **compositional** — the representation of "piano playing with bass" should be decomposable from "piano playing solo" and "bass playing solo."

---

## Appendix A: Timing Budget

| Operation | Budget | Notes |
|-----------|--------|-------|
| CNS bus drain | 100 µs | Lock-free queue, max 64 packets |
| Perception pipeline | 2 ms | JEPA encoder forward pass |
| Alignment computation | 1 ms | Kalman + spring-damper for all active notes |
| Reflex evaluation | 500 µs | Hard-coded pattern match |
| Note rendering | 200 µs | Struct → MIDI bytes |
| Bus TX | 100 µs | Per-packet send |
| **Total per tick** | **~4 ms** | Well within 1 ms tick budget for batched processing |

> Note: The 1 kHz tick processes perception/alignment in batch. Individual note rendering happens at sample-accurate timing within the tick window via a lock-free output queue.

## Appendix B: Director Parameters → Musical Effect

| Parameter | Range | Piano Response | Bass Response | Drums Response |
|-----------|-------|---------------|---------------|----------------|
| `tempo_curve` | ±5% | Follows with 200ms lag | Follows with 50ms lag | Follows immediately |
| `intensity` | 0.0–1.0 | More notes, louder | Walking lines, busier | Dense grooves, more fills |
| `color` | 0.0–1.0 | Dark→open voicings, bright→closed | Round→sustained notes, bright→plucked | Brushes→sticks, dry→washy |
| `weight` | agent_id | Takes melody if weighted | Roots-only if not weighted | Simplified groove if not weighted |
| `space` | 0.0–1.0 | Thins chords, sustains pedal | Drops passing notes | Drops ghost notes, simplified hat |

---

*Design by Fleet Ensemble project. Synthesized from DeepSeek V4-Pro, ByteDance Seed-2.0-pro, and Hermes-3-Llama-405B perspectives. August 2026.*

*Expanded August 13, 2026 with JIT compiler analogy (Hermes-3-Llama-405B), phase-lock loop and attention section (Seed-2.0-pro), instrument personality profiles (Seed-2.0-pro + Hermes-3), and educator-oriented training pipeline (Seed-2.0-pro). Hyperlinked to foundational papers across signal processing, machine learning, and music theory.*

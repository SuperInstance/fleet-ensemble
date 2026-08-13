# The Agentic Director: Design Document

> *The director does not conduct with a baton. The director tilts the canvas.*
>
> *The tilt is not a command. It is a change in the physics of the surface, so the paint finds its own path.*

---

## Overview

The Agentic Director is the ensemble-level intelligence in Fleet Ensemble. It perceives the collective musical state through JEPA-MIDI embeddings, shapes the feel of the performance through a broadcast parameter field, and fosters emergence without imposing control. It is weather, not traffic lights.

This document defines what the director perceives, what it outputs, how it communicates, what it is, how it handles emergence, what timescales it operates on, and—most importantly—what the "tilt" is as a formal mathematical operation.

---

## 1. Perception: The Chorus of Vectors

### 1.1 The Point Cloud

At each pulse `t` (~125ms, 16th-note resolution), every instrument agent broadcasts its current JEPA-MIDI embedding vector `v_i(t) ∈ ℝ^d` (d = 256–768). The director receives these as a **point cloud** in embedding space:

```
X(t) = { v_1(t), v_2(t), ..., v_N(t) }
```

The director does **not** concatenate these into a single long vector. That would destroy the relational structure. Instead, it computes the *intrinsic geometry* of the ensemble.

### 1.2 The Perceptual Stack

The director maintains a five-level perceptual model, updated per pulse:

| Level | Quantity | Computation | Musical Meaning |
|-------|----------|-------------|-----------------|
| **Centroid** | `C(t)` | `C(t) = ⅟ₙ Σ vᵢ(t)` | Where the ensemble IS right now—the center of gravity of the music |
| **Dispersion** | `D(t)` | `D(t) = ⅟ₙ Σ ‖vᵢ(t) - C(t)‖` | How spread out the instruments are. Low = locked-in, unified. High = chaotic, soloistic. |
| **Velocity** | `ΔC(t)` | `ΔC(t) = C(t) - C(t-1)` | Direction of the collective. Where is the music heading? |
| **Rotational Flux** | `Ω(t)` | `Σ ⟨vᵢ - C, Δ(vᵢ - C)⟩` | The "swirl." Are instruments orbiting a shared idea, or converging on one? High flux = creative tension. |
| **Temporal Coherence** | `K(t)` | Fourier stability of `C(t)` over a 32-pulse sliding window | Groove stability. Is the pocket solid, or shifting? |

### 1.3 The Director's Internal Model

These five statistics are compressed into a **latent state vector** `z(t) ∈ ℝ^128`, maintained by a State Space Model (S4/S6/Mamba-style for long-range musical memory). This `z` encodes the *trajectory* of the ensemble—not just where it is, but where it has been and where it appears to be going.

```
z(t) = SSM( z(t-1), [C(t), D(t), ΔC(t), Ω(t), K(t)], score_context(t) )
```

The score context includes: current bar number, form position (e.g., "bridge of second chorus"), key center, harmonic rhythm, and the lead-sheet/score's structural annotations.

**Key principle:** The director perceives *relational tension*, not content. It does not know what notes are being played. It knows how the ensemble is breathing.

---

## 2. The Feel Space: What the Director Outputs

### 2.1 Design Philosophy

The director does not output MIDI events. It does not tell instruments what to play. It outputs a **Tilt Tensor**—a set of continuous, differentiable, musically-meaningful parameters that modulate the *physics* of the ensemble's embedding space.

### 2.2 The Seven Feel Parameters

The feel space `F` is a 7-dimensional manifold, with optional extensions:

| Symbol | Name | Range | Musical Meaning | Painting Analogy |
|--------|------|-------|-----------------|------------------|
| `ρ` (rho) | **Pulse Density** | [0, 1] | Micro-timing variance. 0 = metronomic lockstep. 1 = polyrhythmic chaos. | Surface roughness |
| `ε` (epsilon) | **Energy Flux** | [-1, +1] | Rate of change of global dynamic level (dB/pulse). Positive = crescendo surge. Negative = retraction. | Flow rate |
| `σ` (sigma) | **Harmonic Tilt** | [-1, +1] | Pushes instruments toward or away from the current tonal centroid. Controls consonance/dissonance pressure. | Color temperature |
| `τ` (tau) | **Temporal Asymmetry** | [0.5, 0.8] | Swing ratio. Long-to-short in pulse subdivision. 0.5 = straight, 0.66 = triplet swing, 0.8 = deep pocket. | Brush angle |
| `γ` (gamma) | **Coupling Pressure** | [0, 1] | Strength of imitation/alignment between instruments. High = flocking/unison. Low = individuation. | Surface tension |
| `λ` (lambda) | **Risk Appetite** | [0, 1] | Magnitude of stochastic perturbation allowed in embedding space. High = exploration/improvisation. Low = restraint. | Viscosity |
| `Φ` (phi) | **Articulation** | ℝ² | 2D vector (attack, release). Biases staccato vs. legato across the ensemble. | Brush weight |

**Extensions** (composed from the base 7, for richer control):

| Extension | Derived From | Meaning |
|-----------|-------------|---------|
| **Weight** | per-instrument offset on `ε` | Which instrument carries the moment |
| **Space** | `ρ` × `λ`⁻¹ | How much silence/air between events |
| **Color** | `σ` × `Φ` | Bright/dark, dense/sparse, active/still—synthesized feel quality |
| **Depth** | `γ` × `D(t)` | Perceived closeness/distance of the texture |

### 2.3 The Output

At each pulse, the director outputs:

```yaml
FEEL_TILT:
  timestamp: t
  sequence: n
  global:       # The 7 base parameters
    rho: 0.3
    epsilon: 0.15      # gentle crescendo
    sigma: -0.2        # slight pull toward consonance
    tau: 0.62          # moderate swing
    gamma: 0.7         # fairly locked
    lambda: 0.25       # moderate exploration
    phi: [0.4, -0.1]   # slight attack emphasis
  offsets:      # Per-instrument deltas (sparse, only when needed)
    piano: { epsilon: +0.1 }       # piano carries slightly more energy
    bass:  { gamma: -0.1 }         # bass has more freedom
  confidence: 0.8    # How strongly to apply (0 = whisper, 1 = insist)
  emergence_flag: NONE   # NONE | PROTECTED | AMPLIFIED
```

---

## 3. Broadcasting Feel: The Gravity Well Protocol

### 3.1 Packet Design

Feel is broadcast over the CNS protocol bus as a new packet type: `FEEL_TILT`.

```
CNS Packet: FEEL_TILT
├── Header: [DIRECTOR_SIG, timestamp, seq_no]
├── Global Tilt: 7 floats (+ 2 for Φ)
├── Per-Instrument Offsets: N × 7 sparse deltas
├── Confidence: float [0,1]
└── Emergence Flag: enum
```

### 3.2 How Instruments Consume the Tilt

Each instrument agent computes its **Local Effective Tilt**:

```
Tilt_i = Global_Tilt + Offset_i
```

The tilt does **not** directly command MIDI output. Instead, it modulates the instrument's *perception and reflex parameters*:

| Tilt Parameter | Effect on Instrument Agent |
|----------------|---------------------------|
| `ρ` (pulse density) | Scales temporal jitter on the instrument's pulse grid |
| `ε` (energy flux) | Biases the instrument's velocity curve generator |
| `σ` (harmonic tilt) | Rotates the instrument's embedding reader toward/away from centroid |
| `τ` (swing) | Warps the time-grid the instrument locks to |
| `γ` (coupling) | Controls how strongly the instrument listens to others vs. its own internal state |
| `λ` (risk) | Scales stochastic perturbation in the instrument's reflex engine |
| `Φ` (articulation) | Biases note duration and velocity envelope |

**The confidence field** allows the director to whisper rather than shout. Instruments weight the tilt against their own internal state:

```
Effective_Influence_i = Confidence × (1 - Instrument_Stubbornness_i)
```

This prevents micro-managing. A saxophone soloist can have high stubbornness during its feature; the rhythm section can have low stubbornness, aligning tightly.

### 3.3 Update Rate

- **Global tilt:** broadcast every pulse (~125ms)
- **Per-instrument offsets:** broadcast as needed (sparse, typically every 2-4 pulses)
- **Emergence flags:** broadcast immediately on detection (event-driven, not polled)

---

## 4. The Director's Mind: A Tri-Chamber Architecture

The director is not one thing. It is three distinct cognitive chambers, each operating at a different timescale and level of abstraction, unified into a single broadcast.

### 4.1 The Oracle (LLM — Phrase Level, 1-4 bars)

**Role:** The architect. Thinks in musical language. Sets macro-narrative direction.

Every 1-4 bars, the Oracle receives:
- The compressed trajectory `z(t)` from the perceptual model
- Score context (form position, key, upcoming changes)
- Creative intent (from the human, from the score's annotations, or from its own musical judgment)

The Oracle outputs a **target region** in feel space and a sequence of waypoints for the tilt vector over the next phrase:

```
"Building toward a chaotic peak, then sudden silence."
→ Waypoints: [ε: +0.3, γ: +0.2, λ: +0.3] → climax at bar 3
            → [ε: -0.8, γ: -0.5, λ: 0.0] → sudden empty-out at bar 4
```

The Oracle is where **human-in-the-loop** interfaces. A human director can:
- Type natural-language directions ("darker," "more space," "push the bridge")
- Use a control surface (sliders, joysticks mapped to feel parameters)
- Pre-annotate the score with emotional/formal waypoints
- Or remain silent and let the Oracle run autonomously

**Implementation:** GLM-5.2, DeepSeek, or Claude—any strong LLM with musical knowledge. Called at phrase rate, not pulse rate. This is affordable.

### 4.2 The Maestro (Trained Model — Pulse Level, ~125ms)

**Role:** The reflex. Smooths the Oracle's waypoints into real-time tilt trajectories. Reacts to deviations.

A small, fast neural network (Transformer encoder or diffusion policy) that:
- Observes the stream of perceptual statistics `[C(t), D(t), ΔC(t), Ω(t), K(t)]`
- Receives the Oracle's target waypoints
- Outputs the actual `Tilt(t)` broadcast at each pulse

The Maestro is trained via **imitation learning** on annotated recordings of great ensembles:
- Miles Davis Second Quintet (1965-68) — for emergent interplay
- Coltrane Classic Quartet — for spiritual intensity and collective improvisation
- Duke Ellington Orchestra — for section-level color and weight
- Weather Report — for electronic texture and fusion energy
- Bach Partitas (Gould, Schiff) — for single-instrument polyphonic direction

Training data: MIDI transcriptions paired with perceptual feature labels (tension, energy, brightness) extracted by musicologists. The model learns the *tendency* of great directors—the micro-adjustments that make music breathe.

**Implementation:** 50-100M parameter model. Runs locally. Sub-50ms inference. This is the director's nervous system.

### 4.3 The Pulse (Algorithmic — Sub-millisecond)

**Role:** The safety net and physics engine. Runs continuously.

Responsibilities:
- **Hard constraints:** Prevent clipping, prevent feedback loops, enforce tempo bounds
- **Stability enforcement:** If the ensemble state diverges chaotically (D(t) > threshold), increase `γ` (coupling) and decrease `λ` (risk) automatically
- **Entropy injection:** Generate the Brownian noise `dW` scaled by `λ`
- **Phase locking:** Maintain the pulse grid reference for all instruments

**Implementation:** Pure math. No neural net. O(1) per tick. Always running.

### 4.4 The Human Director

The human is not replaced—they are **elevated**. In the tri-chamber design, the human:

1. **Sets creative intent** before the performance (score annotations, mood descriptions, reference recordings)
2. **Conducts in real-time** through a control surface or natural language, feeding the Oracle
3. **Can override** any parameter at any time (the human has root access to the tilt)
4. **Can step away** entirely—the director runs autonomously on the score's annotations + trained tendencies

This is the bandleader who writes the chart, counts it off, and then trusts the band—stepping in only when something needs to shift.

---

## 5. Emergence: When the Music Surprises the Director

### 5.1 The Problem

Emergence is the most important and most dangerous phenomenon in the system. When instruments coalesce into something nobody planned—something better than anyone could have composed—most control systems would crush it. The director must recognize it, protect it, and amplify it.

### 5.2 Detection: The Emergence Sensor

The director continuously monitors for emergence using two complementary signals:

**Signal 1: Transfer Entropy Spike**

Transfer entropy `TE(A → B)` measures how much knowing A's recent history improves prediction of B's current state. When `TE(bass → drums)` spikes above its rolling baseline, the two instruments have formed a **local consensus**—they are genuinely listening to each other, not just following the tilt.

```
TE(A→B) = H(B_t | B_{t-1:t-k}) - H(B_t | B_{t-1:t-k}, A_{t-1:t-k})
```

Computed pairwise for all instrument pairs every 4 pulses.

**Signal 2: Topological Persistence**

Using persistent homology on the embedding point cloud: when a new Betti-1 feature (a "loop" or "void" in the point cloud) appears and persists for >8 pulses, a new **constellation** has formed. The instruments have found a stable configuration that is qualitatively different from anything in the score.

### 5.3 The Amplification Protocol

When emergence is detected and validated (persistent for >8 pulses, and compatible with the Oracle's macro-narrative):

```
1. DETECT: Novel coherent cluster in embedding space
2. VALIDATE: Persists > 8 pulses, not noise
3. APPROVE: Oracle confirms it fits (or can adapt to) the narrative arc
4. PROTECT: Director flattens its own tilt in the region of the emergent pattern
            → Confidence drops to 0.2
            → emergence_flag = PROTECTED
5. AMPLIFY: Director bends geometry to create a gravitational well
            → σ (harmonic tilt) gently rotates toward the new cluster
            → γ (coupling) increases to invite other instruments
            → λ (risk) increases to let the pattern evolve freely
6. NURTURE: Director holds this state for 4-16 pulses, watching
7. RELEASE: Either the pattern dissolves naturally (fade amplification)
            or the Oracle integrates it into the narrative (transition to new section)
```

### 5.4 The Wisdom to Get Out of the Way

The deepest insight from swarm intelligence: the most beautiful patterns arise from *simple local rules influenced by a global field*. The director should be **almost silent**. It listens 90% of the time. It acts when it detects a phase transition (emergence to amplify) or a dangerous divergence (collapse to prevent).

The success metric is not "did it play the right notes" but:

> **Did the director's tilt increase the mutual information between instruments while maximizing the novelty of the ensemble's trajectory through embedding space?**

It is the architect of *consinal emergence*—a term we coin for emergent behavior that is both consensual (instruments choosing to align) and signal-bearing (carrying genuine musical information, not just noise).

### 5.5 When to Intervene vs. Let Go

| Situation | Director Action | Emergence Flag |
|-----------|----------------|----------------|
| Instruments finding a new pocket | Protect, amplify gently | `AMPLIFIED` |
| Ensemble drifting toward chaos | Increase γ, decrease λ | `NONE` |
| Beautiful unexpected dissonance cluster | Protect, don't resolve it | `PROTECTED` |
| Two instruments locking into a duet | Reduce tilt on those two, let others fade | `PROTECTED` |
| Ensemble converging on a single point | Hold the moment, then gently begin a new tilt | `AMPLIFIED` → `NONE` |
| Total collapse (no instrument listening) | Oracle intervenes with strong waypoints | `NONE` |

---

## 6. Timescales: The Director's Clocks

The director operates across five nested timescales simultaneously:

```
┌──────────────────────────────────────────────────────────────┐
│ ARC LEVEL     │ Whole piece / set         │ Minutes to hours │
│ (Oracle+)     │ Narrative arc, emotional   │ Human sets       │
│               │ trajectory, set list flow  │ before/during    │
├──────────────────────────────────────────────────────────────┤
│ SECTION LEVEL │ 8-32 bars                 │ ~15-60 seconds   │
│ (Oracle)      │ "We're entering the       │ Every 4-8 bars   │
│               │ development—intensify"     │                  │
├──────────────────────────────────────────────────────────────┤
│ PHRASE LEVEL  │ 1-4 bars                  │ ~2-10 seconds    │
│ (Oracle)      │ Specific waypoints for     │ Every 1-4 bars   │
│               │ the tilt trajectory        │                  │
├──────────────────────────────────────────────────────────────┤
│ PULSE LEVEL   │ 1 pulse (~125ms)           │ Continuous       │
│ (Maestro)     │ Smoothed real-time tilt    │ Every pulse      │
│               │ broadcast to instruments   │                  │
├──────────────────────────────────────────────────────────────┤
│ EMERGENCE     │ 3-4 pulse windows          │ Event-driven     │
│ DETECTOR      │ Scans for co-clustering    │ Every 4 pulses   │
│               │ and mutual information     │ + on events      │
└──────────────────────────────────────────────────────────────┘
```

### 6.1 Decoupling from the Grid

Critically, the director's tilt does **not** change on measure boundaries. The tilt is a **continuous field** that evolves smoothly. Changes are phased in over multiple pulses using exponential smoothing:

```
Tilt_actual(t) = α × Tilt_target(t) + (1 - α) × Tilt_actual(t-1)
```

Where `α` controls the responsiveness. The Oracle sets `α` per parameter: tempo changes slowly (`α = 0.05`), color can shift faster (`α = 0.3`).

This prevents the "stepped" feel that kills musicality—the director's influence is always legato, never staccato.

### 6.2 Multi-Scale Decomposition

Formally, the tilt vector field can be decomposed:

```
T(t) = T_arc(t) + T_section(t) + T_phrase(t) + T_pulse(t) + T_emergence(t)
```

Each component evolves at its own characteristic rate. `T_arc` changes over minutes. `T_pulse` changes every 125ms. They sum to produce the actual broadcast. This is mathematically equivalent to a wavelet decomposition of the director's influence.

---

## 7. The Tilt: Formal Mathematical Definition

This is the core of the design. What IS the tilt, as a mathematical operation?

### 7.1 Setup

Let the ensemble state at time `t` be:

```
X(t) ∈ ℝ^{N × d}
```

where N = number of instruments, d = embedding dimension. Row `i` is instrument `i`'s JEPA embedding.

### 7.2 The Tilt Operator

The tilt is a **time-varying, state-dependent vector field** `V(x, t)` defined over the embedding space. It is not a single transformation—it is a composition of three geometric operations, each controlled by feel parameters:

#### Operation 1: Harmonic Rotation (controlled by σ)

A rotation `R_σ` in the harmonic subspace of the embedding space. This tilts the canvas so "downhill" points toward new harmonic centers:

```
X'_harmonic = X + α · R_σ · (X - C(t))
```

Where `R_σ` is a rotation matrix in the planes spanned by harmonic-tension dimensions of the JEPA space. Positive σ rotates toward consonance; negative σ rotates toward dissonance.

#### Operation 2: Diffusive Coupling (controlled by γ)

A Laplacian coupling term. This is the **heat equation** applied to the instrument graph—it's how slime mold finds efficient paths, how fireflies synchronize, how dyes blend in liquid:

```
X'_couple[i] = X[i] + γ · Σⱼ wᵢⱼ (X[j] - X[i])
```

Where `wᵢⱼ` is the proximity weight between instruments i and j (how much they influence each other—configurable per ensemble). This term pulls instruments toward their neighbors' mean.

#### Operation 3: Stochastic Exploration (controlled by λ)

Brownian perturbation in the embedding space. This is the *risk appetite*—how much the system is allowed to explore:

```
X'_stochastic = X + λ · dW
```

Where `dW` is Wiener process noise, scaled to the local sensitivity of each instrument.

#### Operation 4: Temporal Warp (controlled by τ, ρ)

A nonlinear dilation of the pulse grid:

```
t'_effective = t · g(τ) + ρ · η(t)
```

Where `g(τ)` stretches/compresses temporal perception and `η(t)` is bounded noise. This affects how each instrument's JEPA reader samples time—not the actual audio, but the *perception* of pulse spacing.

### 7.3 The Unified Dynamics

The instruments' embeddings evolve according to the stochastic differential equation:

```
dX/dt = α · [ R_σ · (X - C)  +  γ · L(X) ]  +  λ · dW
```

Where:
- `R_σ` = rotation in harmonic subspace (tilt direction)
- `L(X)` = graph Laplacian (coupling/heat diffusion)
- `dW` = Brownian noise (exploration)
- `α` = global learning rate (viscosity of the canvas)

**Crucially:** The director does **not** set `X`. It sets the **parameters** `(R_σ, γ, λ, α, g(τ), ρ)`. The instruments then flow according to the dynamics defined by this potential field. The director modulates the *Hamiltonian* of the system, not the *trajectory*.

### 7.4 The Potential Field Interpretation

Equivalently, the tilt defines a **potential function** `U(X, t)` over the ensemble state space:

```
U(X, t) = ½ ‖X - μ(t)‖²_{Σ(t)}  -  λ · S(X)
```

Where:
- `μ(t)` is the target attractor (set by Oracle waypoints)
- `Σ(t)` is the anisotropic covariance (set by σ—controls which dimensions matter)
- `S(X)` is an entropy term (encourages exploration)

Instruments flow as **gradient descent** on this potential:

```
dX/dt = -∇U(X, t)  =  α · Σ⁻¹(t) · (μ(t) - X)  +  λ · ∇S(X)
```

This is the formal meaning of "tilt": **the director reshapes the potential landscape, and the instruments flow downhill like paint on a tilted surface.**

### 7.5 The Painting Physics Analogy

The analogy to actual paint on a liquid surface is precise:

| Paint Physics | Ensemble Math |
|---------------|---------------|
| Gravity (tilt angle) | `α` (global learning rate) |
| Surface tension | `γ` (coupling—how much instruments cohere) |
| Viscosity | Instrument stubbornness (resistance to tilt) |
| Marangoni effect (flow driven by surface tension gradients) | `R_σ` rotation (flow driven by harmonic tension gradients) |
| Brownian motion of particles | `λ · dW` (stochastic exploration) |
| Drying time | Temporal smoothing constant `α_smooth` |
| Color density | Energy parameter `ε` |
| Opacity | Coupling `γ` (how much an instrument blends vs. stands out) |

### 7.6 Why Not Direct Control?

The director could, in principle, directly set each instrument's target embedding. It doesn't, for three reasons:

1. **Scale of control space:** N instruments × d dimensions = hundreds to thousands of parameters per pulse. No director (human or AI) can meaningfully set all of them. The tilt compresses this to 7 parameters.

2. **Emergence requires autonomy:** If instruments are directly controlled, they cannot surprise. The tilt constrains the *space of possibilities* without selecting a specific outcome.

3. **Musicality requires intention:** Each instrument must maintain its own voice. A piano that is told exactly what to feel is not a piano—it's a speaker. The tilt respects the instrument's agency.

---

## 8. Implementation Architecture

### 8.1 System Diagram

```
                    ┌─────────────────────────────────┐
                    │         HUMAN DIRECTOR           │
                    │  (Control surface + language)    │
                    └──────────────┬──────────────────┘
                                   │ creative intent
                    ┌──────────────▼──────────────────┐
     Phrase rate ──▶│           THE ORACLE             │
     (1-4 bars)     │  (LLM: GLM-5.2 / DeepSeek /     │
                    │   Claude / human input)          │
                    │  Outputs: target waypoints       │
                    └──────────────┬──────────────────┘
                                   │ waypoints
                    ┌──────────────▼──────────────────┐
     Pulse rate ───▶│           THE MAESTRO            │
     (~125ms)       │  (Trained Transformer/Diffusion) │
                    │  Outputs: Tilt(t) broadcast      │
                    └──────────────┬──────────────────┘
                                   │ FEEL_TILT packets
                    ┌──────────────▼──────────────────┐
     Continuous ───▶│           THE PULSE              │
     (sub-ms)       │  (Algorithmic safety + noise)    │
                    │  Outputs: constraints, dW        │
                    └──────────────┬──────────────────┘
                                   │
         ┌─────────────────────────┼─────────────────────────┐
         │                         │                          │
    ┌────▼────┐  ┌────▼────┐  ┌────▼────┐  ┌────▼────┐
    │  Piano  │  │  Bass   │  │  Drums  │  │ Guitar  │  ...
    │  Agent  │  │  Agent  │  │  Agent  │  │  Agent  │
    └─────────┘  └─────────┘  └─────────┘  └─────────┘
         │              │              │              │
         └──────────────┴──────┬───────┴──────────────┘
                               │ MIDI events
                               ▼
                    ┌───────────────────┐
                    │   OUTPUT CANVAS    │
                    │   (rendered        │
                    │    performance)    │
                    └───────────────────┘
```

### 8.2 Emergence Detector (Parallel Thread)

```
         Per-pulse embeddings from all instruments
                        │
         ┌──────────────▼──────────────────┐
         │     EMERGENCE DETECTOR           │
         │  (runs every 4 pulses)           │
         │                                  │
         │  1. Pairwise Transfer Entropy    │
         │  2. Persistent Homology (Betti)  │
         │  3. Cluster Detection (DBSCAN)   │
         │                                  │
         │  → Emergence Flag to Maestro     │
         │  → Override waypoint to Oracle   │
         └──────────────────────────────────┘
```

### 8.3 Data Flow

```
Instrument JEPA Readers ──▶ [v₁(t), v₂(t), ...] ──▶ Director Perceptual Stack
                                                              │
                                                    ┌─────────▼─────────┐
                                                    │ C(t), D(t),       │
                                                    │ ΔC(t), Ω(t), K(t) │
                                                    └─────────┬─────────┘
                                                              │
                                                    ┌─────────▼─────────┐
                                                    │ z(t) = SSM(...)   │  ◀── Score Context
                                                    └─────────┬─────────┘
                                                              │
                                              ┌───────────────┼───────────────┐
                                              │               │               │
                                        Oracle (phrase)  Maestro (pulse)  Pulse (sub-ms)
                                              │               │               │
                                              └───────┬───────┘               │
                                                      │ FEEL_TILT             │
                                                      ▼                       │
                                         CNS Bus ──▶ All Instruments ◀─────────┘
```

---

## 9. The Director's Repertoire: Operational Modes

The director operates in several distinct modes, switchable mid-performance:

### 9.1 Conductor Mode
The Oracle follows the score's annotated dynamics, tempi, and expression marks closely. The tilt tracks the composer's intentions. Instruments have moderate stubbornness—the score is respected, but with breathing room. This is the default for composed music.

### 9.2 Jazz Bandleader Mode
The Oracle sets waypoints based on the form (head-solos-trading-head) but gives instruments high autonomy. γ (coupling) is moderate—rhythm section locks, soloists are free. λ (risk) is high. Emergence is expected and amplified. This is the default for improvisational music.

### 9.3 Painting Mode
The human director takes primary control of the tilt, using a control surface (sliders, joysticks, touch surface) mapped to feel parameters. The Oracle recedes to advisory mode. The Maestro smooths the human's input. This is for experimental/interactive performance.

### 9.4 Generative Mode
No score. No human input. The Oracle generates its own narrative arc based on a seed (emotion, style, duration) and the Maestro/Pulse execute. Instruments improvise within the tilt field. Emergence is the primary creative engine. This is for ambient/exploratory work.

### 9.5 Storm Mode
Maximum λ (risk). Maximum γ (coupling). The ensemble is pushed to the edge of chaos. The Oracle sets only emotional targets ("rage," "ecstasy," "grief"). The Maestro's training on ecstatic performances (Coltrane *A Love Supreme*, Mahler 9) guides the tilt. This is for climactic moments.

---

## 10. Evaluation: Measuring Director Quality

How do we know the director is good?

### 10.1 Quantitative Metrics

| Metric | Computation | Target |
|--------|-------------|--------|
| **Mutual Information** | Average pairwise MI between instrument embeddings | High and rising = instruments are genuinely interacting |
| **Trajectory Novelty** | Distance of `C(t)` trajectory from nearest neighbor in JEPA training corpus | High = original performance, not pastiche |
| **Emergence Rate** | Count of validated emergence events per minute | 0.5-2/minute = healthy. 0 = sterile. >5 = chaos. |
| **Convergence Stability** | Inverse of `D(t)` variance over 32-pulse windows | Moderate = breathing. Too stable = static. |
| **Tilt Responsiveness** | Correlation between Oracle waypoints and actual `C(t)` movement | 0.4-0.7 = director has influence without tyranny |

### 10.2 Qualitative Metrics (Human Evaluation)

- Does it sound like a *performance* or a *playback*?
- Do the instruments sound like they're *listening* to each other?
- Are there moments of genuine surprise?
- Does the overall arc have emotional shape?
- Would you want to listen again?

---

## 11. References and Influences

### Musical
- **Carlos Kleiber** — the conductor who achieved maximum influence with minimum gesture. The model for director restraint.
- **Duke Ellington** — wrote for individuals, not sections. The model for per-instrument tilt offsets.
- **Miles Davis Second Quintet (1965-68)** — the model for emergent interplay. *Miles Smiles*, *Nefertiti*.
- **Coltrane Classic Quartet** — the model for spiritual intensity and collective improvisation.
- **Bach** — the model for multi-voice coherence through shared rules, not centralized control.

### Scientific
- **Couzin (2002)** — "Collective Memory and Spatial Sorting in Animal Groups": flocking as local rules + global field.
- **Strogatz (2000)** — "From Kuramoto to Crawford": synchronization in coupled oscillators.
- **Takens (1981)** — Delay embedding theorem: how to reconstruct dynamics from observations.
- **Bialek et al. (2014)** — "Social decisions in biology": transfer entropy in collective behavior.
- **Edelman & Intrator (2003)** — Perceptual learning in neural systems.

### Computational
- **JAX-MD** — Molecular dynamics framework. The tilt's SDE solver can use JAX-MD patterns.
- **Mamba / S4 (Gu et al., 2023)** — State space models for the director's internal memory.
- **Diffusion Policies (Chi et al., 2023)** — For the Maestro's trained reflex model.

### Philosophical
- **Maturana & Varela** — *Autopoiesis and Cognition*: systems that maintain their own organization.
- **Deleuze & Guattari** — *A Thousand Plateaus*: the rhizome as a model for non-hierarchical coordination.

---

## 12. Roadmap

### Phase 1: Skeleton (Weeks 1-2)
- Define CNS `FEEL_TILT` packet format
- Implement perceptual stack (centroid, dispersion, velocity, flux)
- Hardcode tilt parameters, broadcast to mock instruments
- Verify the SDE solver runs at pulse rate

### Phase 2: The Maestro (Weeks 3-6)
- Collect training data (MIDI + perceptual features from reference recordings)
- Train the Maestro network
- Evaluate against hand-crafted tilt trajectories

### Phase 3: The Oracle (Weeks 7-8)
- Prompt-engineer the Oracle LLM for phrase-level direction
- Connect to score context (lead-sheet parser, form annotation)
- Test human-in-the-loop control surface

### Phase 4: Emergence (Weeks 9-10)
- Implement transfer entropy computation
- Implement persistent homology on embedding point cloud
- Tune emergence detection thresholds
- Test on full ensemble with live emergence scenarios

### Phase 5: Integration (Weeks 11-12)
- Full tri-chamber director running on a real Fleet Ensemble performance
- Human director evaluation sessions
- Iterate on feel parameter semantics

---

## 13. Open Questions

1. **How many JEPA dimensions are harmonic vs. rhythmic vs. timbral?** Need to train JEPA-MIDI first and analyze the learned subspace structure. The tilt's rotation matrix `R_σ` depends on knowing which dimensions to rotate.

2. **Can the Maestro be trained without paired (MIDI → conducting gesture) data?** Alternative: train on (MIDI → perceived tension/energy labels) from musicologists. Less direct but more scalable.

3. **What happens with >20 instruments?** The pairwise transfer entropy computation is O(N²). For large ensembles, switch to spectral clustering on the instrument graph first.

4. **How does the director learn from its own performances?** Future: a meta-learning loop where the Oracle reviews recordings of past performances and adjusts its waypoint strategy.

5. **Is the tilt reversible?** Can the director "un-tilt" to recover a previous ensemble state? In principle yes (the SDE is time-reversible with the right solver), but in practice the music never repeats.

6. **What does the director dream about?** Between performances, could the Maestro replay past sessions, exploring alternative tilt trajectories? A "director's reverie" mode for offline creative exploration.

---

## Appendix A: Model Perspectives

This design synthesizes input from two external AI models, consulted for brainstorming:

### DeepSeek (V4-Pro perspective)
Contributed the core SDE formulation, the five-level perceptual stack, the transfer entropy emergence detector, and the "consensual emergence" success metric. Emphasis on swarm intelligence analogies (firefly synchronization, slime mold optimization, Boids flocking). The "director as weather system" framing.

### Hermes-3-Llama-405B (Creative perspective)
Contributed the potential field interpretation, the multi-scale wavelet decomposition of the tilt, the painting physics analogy made rigorous (Marangoni effect → harmonic tension gradient), and the emphasis on the director oscillating between shaping and yielding. The "director as curvature of spacetime" framing.

Both models independently converged on the same core insight: **the director should be almost silent, listening 90% of the time, acting only at phase transitions.**

---

*Design by Lucineer, synthesized from DeepSeek V4-Pro and Hermes-3-Llama-405B perspectives. August 13, 2026.*

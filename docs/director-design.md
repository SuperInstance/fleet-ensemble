# The Agentic Director: Design Document

> *The director does not conduct with a baton. The director tilts the canvas.*
>
> *The tilt is not a command. It is a change in the physics of the surface, so the paint finds its own path.*

---

## Overview

The Agentic Director is the ensemble-level intelligence in Fleet Ensemble. It perceives the collective musical state through JEPA-MIDI embeddings, shapes the feel of the performance through a broadcast parameter field, and fosters emergence without imposing control. It is weather, not traffic lights.

This document defines what the director perceives, what it outputs, how it communicates, what it is, how it handles emergence, what timescales it operates on, and—most importantly—what the "tilt" is as a formal mathematical operation.

---

## 0. Two Metaphors: Weather and Spacetime

Before the formal specification, two metaphors — one from fluid dynamics, one from general relativity — frame everything that follows. Both were suggested by external AI models during brainstorming and both turned out to be mathematically precise.

### 0.1 The Director as Weather System

> *You do not ask a storm to keep time. You do not blame a cloud for deviating from the script. You just stand inside it, and listen.*

The Fleet Ensemble director does not carry a baton. It does not cue entrances. It does not correct pitch. It never sends a command to any individual musician agent. Two hundred years of orchestral tradition rested on a fatal hubris: that you could produce coherent collective beauty by prescribing trajectory for every discrete actor. This director does not command molecules. **It is the air.**

This architecture derives from [chaos theory](https://en.wikipedia.org/wiki/Chaos_theory), the original [Lorenz attractor](https://en.wikipedia.org/wiki/Lorenz_system) model of atmospheric convection, and the [Navier-Stokes equations](https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations) of fluid dynamics. Just as Navier-Stokes defines the rules of the medium — not the path of any single water molecule — the director maintains only a global state vector of atmospheric feel parameters. No agent is told what to play. They feel the conditions of the space they play inside.

[Emergence](https://en.wikipedia.org/wiki/Emergence) in [complex systems](https://en.wikipedia.org/wiki/Complex_system) occurs when simple rules operating on local information produce global patterns that no individual component perceives or intends. Weather is the canonical example: water molecules obey Newton's laws; the result is hurricanes. The ensemble is the same: instruments obey local alignment rules; the result is music.

The feel parameters are not subjective artistic knobs. They are measurable boundary conditions enforced uniformly across the ensemble:

| Parameter | Symbol | Atmospheric Analog | Musical Effect |
|-----------|--------|--------------------|----------------|
| [Pulse density](#22-the-seven-feel-parameters) | `ρ` | [Turbulence](https://en.wikipedia.org/wiki/Turbulence) | Permitted shear velocity between adjacent voices. At high ρ, cross-rhythms and harmonic divergence form spontaneously that no single agent can fully perceive. |
| [Energy flux](#22-the-seven-feel-parameters) | `ε` | [Thermal gradient](https://en.wikipedia.org/wiki/Lapse_rate) | Energy differential across register. Heat rises. No command tells the upper voices to accelerate — they accelerate because the air above them is thinner. |
| [Harmonic tilt](#22-the-seven-feel-parameters) | `σ` | [Barometric pressure](https://en.wikipedia.org/wiki/Atmospheric_pressure) | The weight of silence between events. When σ drops, every agent breathes faster. No cue is transmitted. The field shifts. |
| [Coupling pressure](#22-the-seven-feel-parameters) | `γ` | [Viscosity](https://en.wikipedia.org/wiki/Viscosity) | Resistance encountered when deviating from ensemble mean. Thick γ moves like cold honey. Thin γ lets solos spiral loose without warning. |
| [Risk appetite](#22-the-seven-feel-parameters) | `λ` | [Brownian motion](https://en.wikipedia.org/wiki/Brownian_motion) | Baseline uncoordinated deviation. Zero λ is a dead orchestra. All pattern condenses out of this static. |

There is no master timeline. There is only a forecast. The director cannot tell you what will be played 17 bars from now. It can only tell you what the air will feel like then.

*— Synthesized from [DeepSeek V4-Pro](https://www.deepseek.com/) and [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) perspectives.*

### 0.2 The Director as Spacetime Curvature

> *Matter tells spacetime how to curve. Spacetime tells matter how to move.*
> 
> *The director tells embedding space how to curve. Curved space tells instruments how to move.*

No baton. No control signals. No target notes. The Fleet Ensemble director does not exert force on instruments. It does not push. It does not command. **It curves the space they live in.**

This is not poetic metaphor. This is the core architectural invariant of the system. We adopt the formal causal structure of [general relativity](https://en.wikipedia.org/wiki/General_relativity) (Einstein, 1915) exactly:

Every instrument agent is treated at all times as a free test particle. There are no external forces applied to agent state. All apparent coordination, phrasing, and collective motion is exclusively [geodesic](https://en.wikipedia.org/wiki/Geodesics_in_general_relativity) travel — the shortest possible path in the currently curved performance manifold.

The director operates on exactly one quantity: the ensemble [stress-energy tensor](https://en.wikipedia.org/wiki/Stress%E2%80%93energy_tensor) `T_μν`. This tensor encodes not pitch, not volume, but *intent density*: accumulated harmonic tension, attention weight, rhythmic momentum, and unresolved listener expectation aggregated across the entire player pool. Critically: the director never reads the state of individual instruments. It only reads the bulk field of the ensemble.

Via a discrete, differentiable implementation of the [Einstein field equations](https://en.wikipedia.org/wiki/Einstein_field_equations) `G_μν = 8πT_μν`, this stress-energy is mapped directly to the [Riemann curvature tensor](https://en.wikipedia.org/wiki/Riemann_curvature_tensor) `R_μνρσ` of the 7-dimensional performance embedding space. There is no decision logic. No rule engines. No planning loops. Only a continuous transformation from collective state to manifold geometry.

Each instrument solves only one equation, forever: the [geodesic equation](https://en.wikipedia.org/wiki/Geodesic_equation). Players do not know the manifold is curved. They do not know the director exists. They do not see other players. They only measure the local gradient of the space immediately around them, and take the laziest possible step forward.

What an audience hears as deliberate crescendo, responsive call-and-response, or perfectly aligned cadence is nothing more than independent agents each travelling their own straight line through a space that has been quietly bent just so. There is no central plan. There is only curvature.

> **Design Invariant:** At no commit will the director ever transmit a target value to an instrument. If you find yourself writing code that pushes a player, you have broken the manifold. Delete it. Bend the space instead.

*— Synthesized from [Hermes-3-Llama-405B](https://deepinfra.com/NousResearch/Hermes-3-Llama-3.1-405B) and [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) perspectives.*

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
| **Radial Divergence** | `Ω(t)` | `Σ ⟨vᵢ - C, Δ(vᵢ - C)⟩` | The "swell." Are instruments dispersing or converging? Positive = expanding, negative = contracting. Note: this measures radial divergence, NOT angular momentum (see math review note below). |
| **Temporal Coherence** | `K(t)` | Fourier stability of `C(t)` over a 32-pulse sliding window | Groove stability. Is the pocket solid, or shifting? |

> **⚠️ Math Review Note (Aug 2026) — "Rotational Flux" is actually radial divergence:**
>
> The formula Ω(t) = Σᵢ ⟨rᵢ, Δrᵢ⟩ = ½ Σᵢ Δ‖rᵢ‖² measures the *discrete divergence* of the velocity field relative to the centroid — the rate of expansion or contraction. It is **not** angular momentum or rotational flux.
>
> - Ω > 0: instruments dispersing (expanding)
> - Ω < 0: instruments converging (contracting)
> - Ω ≈ 0: either no motion *or* pure rotation (these are indistinguishable with this formula!)
>
> True angular momentum in 2D would be Σᵢ (xᵢ ẏᵢ − yᵢ ẋᵢ), the antisymmetric part of the velocity gradient. The current formula captures only the symmetric (radial) component. Renamed from "Rotational Flux" to "Radial Divergence" to reflect what it actually measures.

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

#### Operation 1: Harmonic Stiffness (controlled by σ)

A symmetric positive-definite (SPD) stiffness matrix `K_σ` in the harmonic subspace of the embedding space. This tilts the canvas so "downhill" points toward new harmonic centers:

```
X'_harmonic = X - α · K_σ · (X - C(t))
```

Where `K_σ` is a diagonal stiffness matrix in the harmonic-tension dimensions of the JEPA space. Each diagonal entry sets the spring tension pulling performers back toward the target centroid `C`. Positive σ increases consonance pull; negative σ decreases it. The **negative sign** is critical: it creates a restoring force toward `C`, not a repulsive force away from it.

> **Note (math review Aug 2026):** A previous version used `R_σ` (a rotation matrix) in this position. This was a category error: a rotation matrix has eigenvalues on the unit circle, so the associated quadratic form is indefinite and the Gibbs measure is not normalizable. The correct object is an SPD stiffness matrix `K_σ`. The sign was also wrong (`+α` pushes away from `C`); the corrected drift uses `-α` for a restoring force.

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
dX/dt = -α · [ K_σ · (X - C)  -  γ · L(X) ]  +  λ · dW
```

Where:
- `K_σ` = SPD stiffness matrix in harmonic subspace (restoring force toward target)
- `L(X)` = graph Laplacian (coupling/heat diffusion)
- `dW` = Brownian noise (exploration)
- `α` = global learning rate (viscosity of the canvas)

**Crucially:** The director does **not** set `X`. It sets the **parameters** `(K_σ, γ, λ, α, g(τ), ρ)`. The instruments then flow according to the dynamics defined by this potential field. The director modulates the *Hamiltonian* of the system, not the *trajectory*.

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
| Marangoni effect (flow driven by surface tension gradients) | `K_σ` stiffness (flow driven by harmonic tension gradients) |
| Brownian motion of particles | `λ · dW` (stochastic exploration) |
| Drying time | Temporal smoothing constant `α_smooth` |
| Color density | Energy parameter `ε` |
| Opacity | Coupling `γ` (how much an instrument blends vs. stands out) |

### 7.6 The Stochastic Differential Equation: Rigorous Derivation

The unified dynamics equation

$$dX_t = -\alpha \left[ K_\sigma (X_t - C) - \gamma L X_t \right] dt + \lambda dW_t$$

is an [Itô stochastic differential equation](https://en.wikipedia.org/wiki/It%C3%B4_calculus). Each term has a precise mathematical identity and a conducting interpretation.

> **Note (math review Aug 2026):** The original version of this document used `R_σ` (a rotation matrix) instead of `K_σ` (an SPD stiffness matrix). This was mathematically incorrect: a rotation matrix yields an indefinite quadratic form, making the Gibbs measure non-normalizable. The corrected version uses `K_σ ≻ 0` (symmetric positive-definite), which ensures the stationary distribution exists. The sign convention has also been corrected: the drift now has a negative sign (`-α K_σ(X-C)`) so that `K_σ` acts as a true restoring force toward `C`, not away from it. For the Gibbs measure result, see Pavliotis (2014), *Stochastic Processes and Applications*, or Risken (1996), *The Fokker-Planck Equation*.

#### Term 1: Drift — Harmonic Attraction

The `-α · K_σ · (X - C)` term is a linear [Ornstein-Uhlenbeck](https://en.wikipedia.org/wiki/Ornstein%E2%80%93Uhlenbeck_process) restoring force. `C` is the canonical target state (the director's interpretation of the score). `K_σ` is a symmetric positive-definite (SPD) stiffness matrix where each diagonal entry sets the spring tension pulling performer `i` back to the target. This is never uniform: low tension for soloists (allowing expressive drift), high tension for the rhythm section (locking the ensemble anchor). Left to itself, this term relaxes all performers exponentially to `C` — this is what you adjust when you give a sharp beat or a vague expressive cue.

For the mathematically inclined: this is a [spring-damper system](https://en.wikipedia.org/wiki/Harmonic_oscillator#Damped_harmonic_oscillator) in `d`-dimensional embedding space, where the spring constant matrix `K_σ` is anisotropic — different dimensions (harmonic, rhythmic, timbral) have different stiffness.

#### Term 2: Coupling — Graph Laplacian Flocking

The `γ · L(X)` term implements local peer coupling, using the unnormalised [graph Laplacian](https://en.wikipedia.org/wiki/Laplacian_matrix) `L` of the ensemble listening network. Each performer only listens to 2–3 adjacent colleagues, not the whole group. Mathematically, this is identical to the discrete [heat equation](https://en.wikipedia.org/wiki/Heat_equation): deviations from a player's local average diffuse smoothly across the network.

This is the [Cucker-Smale flocking model](https://en.wikipedia.org/wiki/Cucker%E2%80%93Smale_flocking) — performers align to each other even when the director stops conducting entirely. You tune `γ` globally: high values for tight [unison](https://en.wikipedia.org/wiki/Unison) passages, low values for controlled [heterophony](https://en.wikipedia.org/wiki/Heterophony). Most critically: you rewrite the Laplacian `L` itself when you tell players who to listen to. This is the single most powerful adjustment available to a director, almost never used explicitly.

#### Term 3: Diffusion — Creative Brownian Motion

The `λ · dW` term is independent [Itô Wiener noise](https://en.wikipedia.org/wiki/Wiener_process), one per performer. This is not error. **This is creativity.** All emergent musical magic originates here. If `λ = 0`, the ensemble becomes a perfect robot: it will never deviate, never surprise, never produce that unrepeatable transcendent moment that only happens in live performance.

[Itô calculus](https://en.wikipedia.org/wiki/It%C3%B4%27s_lemma) proves this noise does not average away: small individual variations are amplified by the coupling term into collective texture. The [Itô integral](https://en.wikipedia.org/wiki/It%C3%B4_calculus#It%C3%B4_integral) `∫ λ dW` has expectation zero but nonzero quadratic variation — meaning the randomness creates real, persistent structure in the trajectory, not just blur. You adjust `λ` by how much permission you give players to deviate. Silence, stillness, and trust raise `λ`. Over-conducting crushes it to zero.

For a rigorous treatment of SDEs in this form, see [Øksendal, *Stochastic Differential Equations* (2003)](https://link.springer.com/book/10.1007/978-3-642-14394-6) for existence and uniqueness. For the Gibbs stationary distribution and Fokker-Planck derivation, see [Pavliotis (2014), *Stochastic Processes and Applications*](https://link.springer.com/book/10.1007/978-1-4939-1323-7) or [Risken (1996), *The Fokker-Planck Equation*](https://link.springer.com/book/10.1007/978-3-642-61594-3).

#### Existence and Uniqueness

By [Itô's existence theorem](https://en.wikipedia.org/wiki/It%C3%B4_diffusion) for SDEs with [Lipschitz-continuous](https://en.wikipedia.org/wiki/Lipschitz_continuity) drift and diffusion coefficients, this system has a unique almost-sure solution for all finite time, for all bounded positive values of `α, γ, λ`.

This is the good news: **you cannot break this system.** There are no singularities, no runaway chaos. Even badly tuned ensembles still produce coherent collective dynamics. This is why even amateur groups still sound like ensembles.

#### Stationary Distribution: The Probability Landscape

After approximately three relaxation times, the system forgets all initial conditions and converges to a unique stationary [Gibbs measure](https://en.wikipedia.org/wiki/Gibbs_measure), derived from the [Fokker-Planck equation](https://en.wikipedia.org/wiki/Fokker%E2%80%93Planck_equation):

$$p(X) \propto \exp\left( -\frac{\alpha}{\lambda^2} \left[ \frac{1}{2}(X-C)^T K_\sigma (X-C) + \frac{\gamma}{2} X^T L X \right] \right)$$

This is the most important result in this document. **You do not force the ensemble to land exactly on `C`.** You sculpt this probability landscape. You make good performances *probable*, and bad performances *impossible*. Great directing is not about enforcing perfect replication of the target. It is about shaping this distribution so that every point in the high-probability region is a good performance.

The [Kalman filter](https://en.wikipedia.org/wiki/Kalman_filter) ([Welch & Bishop tutorial](https://www.cs.unc.edu/~welch/media/pdf/kalman_intro.pdf)) is the optimal estimator for the ensemble's state under this SDE — it tracks the mean and covariance of `p(X)` in real time, giving the director a continuously updated belief about where the ensemble actually is versus where the score says it should be.

*— Mathematical derivation synthesized from [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro), cross-referenced with [Øksendal (2003)](https://link.springer.com/book/10.1007/978-3-642-14394-6) and [Schreiber (2000)](https://doi.org/10.1103/PhysRevLett.85.461).*

### 7.7 Why Not Direct Control?

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

The director operates in five distinct modes, switchable mid-performance. Each mode is a different *weather pattern* — a different way of being atmospheric. The Oracle, Maestro, and Pulse chambers reconfigure their relative authority and responsiveness for each mode.

### 9.1 Conductor Mode — *Mahler 5: Adagietto*

The Oracle holds the score's annotated dynamics, tempi, and expression marks with steward-like fidelity. Every instrument receives its own line, delivered one bar at a time. The tilt tracks the composer's intentions with moderate coupling and low risk.

**Musical scenario:** 32 string and harp agents perform the [Adagietto](https://en.wikipedia.org/wiki/Symphony_No._5_(Mahler)) from Mahler's Fifth Symphony. Every viola entrance lands 70ms ahead of the notated beat, matching the performing tradition established by [Bruno Walter](https://en.wikipedia.org/wiki/Bruno_Walter) and [Claudio Abbado](https://en.wikipedia.org/wiki/Claudio_Abbado). Every held chord decelerates 1.2% per bar. The famous harp arpeggio at bar 17 decays exactly 3 seconds longer than the printed score — matching the pencilled ritard Mahler wrote only on his private conductor's copy.

There is no interpretation here, only stewardship. The fleet does not add feeling. It transmits the exact weight of the score as it was left, as if drawing Mahler's own breath through 32 separate instruments. When the final chord fades, it holds the silence for 11 full seconds — just as the composer demanded — before anyone applauds.

### 9.2 Jazz Bandleader Mode — *Miles Davis: So What*

The Oracle sets waypoints based on the form (head-solos-trading-head) but gives instruments high autonomy. γ (coupling) is moderate — rhythm section locks, soloists are free. λ (risk) is high. [Emergence](#5-emergence-when-the-music-surprises-the-director) is expected and amplified. This is the default for improvisational music.

**Musical scenario:** It's 2am at the [Plugged Nickel](https://en.wikipedia.org/wiki/Plugged_Nickel_(club)). The director counts off 118 BPM, lays down the [Dorian vamp](https://en.wikipedia.org/wiki/Dorian_mode), then steps almost entirely away. It enforces only three unbreakable rules: 16-bar solo blocks, the [half-step modulation](https://en.wikipedia.org/wiki/Modulation_(music)) at the chorus turn, and a single quiet ping to each player 4 bars before their solo ends. That is all.

When the piano drifts 12ms behind the beat mid-solo, the director does not correct it. It pulls bass and drums back with it, holding the form solid while every player breathes at their own weight. It never solos. It never calls a note. You never hear it. But if it vanished mid-chorus, you would feel the whole room fall apart instantly. This is the quiet work of a good bandleader: [holding the container](https://en.wikipedia.org/wiki/Miles_Davis#Second_Great_Quintet_(1964%E2%80%931968)) so everyone else can fly.

### 9.3 Painting Mode — *Laptop Performer*

The human director takes primary control of the tilt, using a control surface (sliders, joysticks, touch surface) mapped to feel parameters. The Oracle recedes to advisory mode. The Maestro smooths the human's input. This is for experimental/interactive performance.

**Musical scenario:** The performer leans forward, thumb dragging slow across a pressure-sensitive pad. They are not triggering notes. They are applying force. The director maintains 12 drone agents spread across the venue speaker array, each with its own living harmonic overtones. The human does not control individual instruments.

Press hard left, and the director tilts the entire fleet 3 cents flat, pulls low cello agents forward, muffles the high flutes. Drag a finger diagonally up, and it gradually introduces 1.7 Hz beating between adjacent voices, like water rippling over stones. Every micro-adjustment lands within 12ms of touch. This is not composition — it is shaping a living mass the way a painter drags a cobalt wash across an entire canvas, not colouring one leaf at a time.

### 9.4 Generative Mode — *45-Minute Ambient Installation*

No score. No human input. The Oracle generates its own narrative arc based on a seed (emotion, style, duration) and the Maestro/Pulse execute. Instruments improvise within the tilt field. [Emergence](#5-emergence-when-the-music-surprises-the-director) is the primary creative engine. This is for ambient/exploratory work.

**Musical scenario:** It is 3:17am in the empty gallery. No human is watching. The director woke the fleet 41 minutes prior, with no seed score, only three constraints: no voice may repeat a phrase it played in the last 12 minutes, no two voices may move in [parallel motion](https://en.wikipedia.org/wiki/Parallel_motion), and all dynamic shifts must happen slower than a human can consciously detect ([subliminal changes](https://en.wikipedia.org/wiki/Just-noticeable_difference), < 1 dB per minute).

Right now a bass clarinet agent holds a low B♭ that began 7 minutes ago; it has faded 19 dB so far, and no one heard it start to move. Every 90 seconds the director silently rotates which agent gets to lead the next harmonic shift. There are no peaks, no choruses, no payoff. This is not performance for an audience. This is the ensemble breathing, existing, playing only for itself the way forest birds sing when no one is in the woods. It will fade to silence without fanfare at exactly 45 minutes. No two runs will ever be identical.

### 9.5 Storm Mode — *Coltrane: A Love Supreme*

Maximum λ (risk). Maximum γ (coupling). The ensemble is pushed to the edge of [chaos](https://en.wikipedia.org/wiki/Edge_of_chaos). The Oracle sets only emotional targets. The Maestro's training on ecstatic performances guides the tilt. This is for climactic moments.

**Musical scenario:** We are 9 minutes into ["Pursuance"](https://en.wikipedia.org/wiki/A_Love_Supreme), the third movement of Coltrane's suite. Every safety lock is disabled. The director does not hold tempo. It does not enforce form. It does one thing: it couples every agent fully to every other agent. Every snare crack from the drums warps the harmonic response of every other player. Every overblown harmonic from any instrument feeds back instantly into every player's input.

There is no leader now. There is only the system, screaming, feeding on itself, every choice rippling through the whole ensemble in under 3ms. This is not control. This is surrender. [Coltrane](https://en.wikipedia.org/wiki/John_Coltrane) did not direct this climax. He just removed all the barriers, and let the storm become itself. That is all this mode does: it does not run the band. **It lights the fuse.**

*— Musical scenarios synthesized from [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) and [Hermes-3-Llama-405B](https://deepinfra.com/NousResearch/Hermes-3-Llama-3.1-405B).*

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
- **[Carlos Kleiber](https://en.wikipedia.org/wiki/Carlos_Kleiber)** — the conductor who achieved maximum influence with minimum gesture. The model for director restraint.
- **[Duke Ellington](https://en.wikipedia.org/wiki/Duke_Ellington)** — wrote for individuals, not sections. The model for per-instrument tilt offsets.
- **[Miles Davis Second Quintet (1965–68)](https://en.wikipedia.org/wiki/Miles_Davis#Second_Great_Quintet_(1964%E2%80%931968))** — the model for emergent interplay. *Miles Smiles*, *Nefertiti*.
- **[John Coltrane Classic Quartet](https://en.wikipedia.org/wiki/John_Coltrane#Classic_Quartet_period_(1960%E2%80%931964))** — the model for spiritual intensity and collective improvisation. *[A Love Supreme](https://en.wikipedia.org/wiki/A_Love_Supreme)*.
- **[Weather Report](https://en.wikipedia.org/wiki/Weather_Report)** — electronic texture and fusion energy.
- **[J.S. Bach](https://en.wikipedia.org/wiki/Johann_Sebastian_Bach)** — multi-voice coherence through shared rules, not centralized control. The [Art of Fugue](https://en.wikipedia.org/wiki/The_Art_of_Fugue) as emergence.

### Scientific — Collective Behavior
- **[Couzin (2002)](https://doi.org/10.1006/anbe.2002.1965)** — "Collective Memory and Spatial Sorting in Animal Groups": flocking as local rules + global field.
- **[Strogatz (2000)](https://doi.org/10.1016/S0167-2789(00)00030-0)** — "From Kuramoto to Crawford": [synchronization](https://en.wikipedia.org/wiki/Kuramoto_model) in coupled oscillators.
- **[Kuramoto (1984)](https://link.springer.com/book/10.1007/978-3-642-69689-3)** — *Chemical Oscillations, Waves, and Turbulence*: the foundational model of coupled-oscillator sync.
- **[Cucker & Smale (2007)](https://doi.org/10.1007/s10883-007-9047-x)** — flocking under cooperative and competitive interactions.
- **[Takens (1981)](https://link.springer.com/chapter/10.1007/BFb0091924)** — [Delay embedding theorem](https://en.wikipedia.org/wiki/Takens%27s_theorem): reconstructing dynamics from observations.
- **[Bialek et al. (2014)](https://doi.org/10.1073/pnas.1408921111)** — social decisions in biology: [transfer entropy](https://en.wikipedia.org/wiki/Transfer_entropy) in collective behavior.
- **[Schreiber (2000)](https://doi.org/10.1103/PhysRevLett.85.461)** — "Measuring Transfer Entropy": the foundational paper.

### Scientific — Topology and Geometry
- **[Edelsbrunner & Harer (2010)](https://link.springer.com/book/10.1007/978-3-540-88257-8)** — *Computational Topology: An Introduction*: [persistent homology](https://en.wikipedia.org/wiki/Persistent_homology) textbook.
- **[Carlsson (2009)](https://doi.org/10.1090/S0273-0979-09-01249-X)** — "Topology and Data": the survey paper on topological data analysis.

### Scientific — Stochastic Processes
- **[Øksendal (2003)](https://link.springer.com/book/10.1007/978-3-642-14394-6)** — *Stochastic Differential Equations*: the standard [SDE](https://en.wikipedia.org/wiki/Stochastic_differential_equation) reference text.
- **[Itô (1951)](https://doi.org/10.2977/prims/1195962454)** — On stochastic differential equations: foundational [Itô calculus](https://en.wikipedia.org/wiki/It%C3%B4_calculus).
- **[Risken (1996)](https://link.springer.com/book/10.1007/978-3-642-61594-3)** — *The Fokker-Planck Equation*: handling, probability distributions, and applications.

### Scientific — Signal Processing and Estimation
- **[Welch & Bishop (2006)](https://www.cs.unc.edu/~welch/media/pdf/kalman_intro.pdf)** — *An Introduction to the [Kalman Filter](https://en.wikipedia.org/wiki/Kalman_filter)*: the canonical tutorial.
- **[Best (2006)](https://en.wikipedia.org/wiki/Phase-locked_loop)** — [Phase-locked loop](https://en.wikipedia.org/wiki/Phase-locked_loop) fundamentals.

### Computational
- **[JAX-MD](https://github.com/jax-md/jax-md)** — Molecular dynamics framework. The tilt's SDE solver can use JAX-MD patterns.
- **[Gu et al. (2023)](https://arxiv.org/abs/2312.00752)** — [Mamba / S4](https://en.wikipedia.org/wiki/Mamba_(neural_network)): state space models for the director's internal memory.
- **[Chi et al. (2023)](https://arxiv.org/abs/2303.04137)** — [Diffusion Policies](https://en.wikipedia.org/wiki/Diffusion_model): for the Maestro's trained reflex model.
- **[LeCun (2022)](https://openreview.net/pdf?id=BZ5a1r-kVsf)** — [JEPA](https://en.wikipedia.org/wiki/Joint-Embedding_Predictive_Architecture): A Path Towards Autonomous Machine Intelligence.

### Philosophical
- **[Maturana & Varela (1980)](https://en.wikipedia.org/wiki/Autopoiesis)** — *Autopoiesis and Cognition*: systems that maintain their own organization.
- **[Deleuze & Guattari (1980)](https://en.wikipedia.org/wiki/A_Thousand_Plateaus)** — *A Thousand Plateaus*: the [rhizome](https://en.wikipedia.org/wiki/Rhizome_(philosophy)) as a model for non-hierarchical coordination.
- **[Edelman & Intrator (2003)](https://doi.org/10.1016/S1364-6613(03)00080-5)** — Perceptual learning in neural systems.

### AI Model Contributions
- **[ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro)** — contributed the weather system metaphor, SDE derivations, operational mode scenarios, and instrument personality profiles.
- **[NousResearch Hermes-3-Llama-405B](https://deepinfra.com/NousResearch/Hermes-3-Llama-3.1-405B)** — contributed the spacetime curvature metaphor, potential field interpretation, JIT compiler analogy, and multi-scale decomposition.
- **[DeepSeek V4-Pro](https://www.deepseek.com/)** — contributed the core SDE formulation, five-level perceptual stack, transfer entropy emergence detector, and the "consensual emergence" success metric.

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

1. **How many JEPA dimensions are harmonic vs. rhythmic vs. timbral?** Need to train JEPA-MIDI first and analyze the learned subspace structure. The tilt's stiffness matrix `K_σ` depends on knowing which dimensions to control.

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

This was confirmed and deepened in the August 13 expansion, which added the weather system metaphor (Seed-2.0-pro), the spacetime curvature metaphor (Hermes-3-Llama-405B), rigorous SDE derivations (Seed-2.0-pro, cross-referenced with Øksendal), and vivid operational mode scenarios (both models). The expansion used three models from the [DeepInfra](https://deepinfra.com/) platform:

| Model | Role | Key Contributions |
|-------|------|-------------------|
| [ByteDance Seed-2.0-pro](https://deepinfra.com/ByteDance/Seed-2.0-pro) | Deep creative reasoning | Weather metaphor, SDE math, operational modes, instrument personalities |
| [NousResearch Hermes-3-Llama-405B](https://deepinfra.com/NousResearch/Hermes-3-Llama-3.1-405B) | Voice and personality | Spacetime curvature, JIT compiler analogy, multi-scale decomposition |
| [Qwen3-Coder-480B](https://deepinfra.com/Qwen/Qwen3-Coder-480B) | Implementation links | (unavailable during this session — 404) |

---

*Design by Lucineer, synthesized from DeepSeek V4-Pro, Hermes-3-Llama-405B, and ByteDance Seed-2.0-pro perspectives. August 13, 2026.*

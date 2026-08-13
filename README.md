# Fleet Ensemble

**An agentic performance system where MIDI tracks become living instruments that coalesce under a director's feel.**

*Like fine paint on liquid white base paint — each color finds its edge in relation to the others, not by command, but by resonance.*

---

## The Idea

Every MIDI track can be rendered as an **agentic instrument** — a player with its own perception, its own reflexes, its own voice. An **agentic director** performs the rendering synoptically, shaping the feel of the whole ensemble. The instruments align themselves to the director's feel the way a jazz quartet locks into a pocket — not by following a click track, but by listening.

This is the agentic compiler pattern applied to music. Instead of compiling source code to machine code, we compile a musical score to a live performance. Each instrument is an agent. The director is the orchestrator. The canvas is the output.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   AGENTIC DIRECTOR                           │
│   Hears the whole ensemble. Shapes the feel.                 │
│   "Lay back. Darker. More space. Push the bridge."           │
│   Operates on the ENSEMBLE level — not individual notes.     │
└──────────────────┬──────────────────────────────────────────┘
                   │ feel parameters (tempo curve, dynamic shape,
                   │ intensity envelope, color, weight)
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
│              Each instrument hears the others                  │
│              and adjusts its own performance                   │
│              in real time                                      │
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

## The Agentic Compiler → Agentic Performer Analogy

| Compiler Concept | Performance Concept |
|-----------------|---------------------|
| Source code | Musical score / MIDI tracks |
| Lexer | JEPA pulse parser (reads the feel) |
| Parser | Director interprets form and intent |
| AST | Ensemble arrangement (who plays what when) |
| Optimization passes | Instrument agents adjust to each other |
| Code generation | MIDI event generation (real-time) |
| Linker | Mix — all instruments coalesce into one output |
| Binary | The recording — frozen, but it was alive when it happened |

## The Canvas Metaphor

Liquid white base paint is the ground. Each instrument is a color poured onto the surface. The colors don't mix mechanically — they find their own edges through surface tension, viscosity, density. The director doesn't tell each color where to go. The director tilts the canvas.

- **The base paint** is the JEPA embedding space — the shared musical ground
- **Each color** is an instrument agent with its own timbre, register, and behavior
- **The tilt** is the director's feel parameters — tempo curve, intensity, color, weight
- **The painting** is what emerges — unpredictable, alive, coalesced

## Relation to Fleet JEPA-MIDI

| Fleet JEPA-MIDI | Fleet Ensemble |
|----------------|---------------|
| One soloist improvising | Full ensemble performing |
| LLM thinks, JEPA feels, algorithms execute | Director shapes, instruments align, canvas emerges |
| Single-track real-time generation | Multi-track agentic performance |
| Internal feedback loop | Inter-agent communication via CNS protocol |

Fleet Ensemble uses Fleet JEPA-MIDI's embedding space as its shared language. Each instrument agent has its own JEPA reader. The director operates on the ensemble-level embedding — the sum of all instruments' current states.

## The Director

The director is not a conductor with a baton. The director is a painter who tilts the canvas.

Director inputs:
- Ensemble JEPA embedding (what does the whole sound feel like right now?)
- Score context (where are we in the form?)
- Creative intent (what should happen next?)

Director outputs (feel parameters broadcast to all instruments):
- **Tempo curve** — micro-adjustments to pulse (not a static BPM, a living tempo)
- **Dynamic shape** — intensity envelope across the ensemble
- **Color** — bright/dark, dense/sparse, active/still
- **Weight** — which instrument carries the moment
- **Space** — how much silence between events

## The Instruments

Each instrument agent has:

1. **A voice** — its MIDI program, register, timbral preferences
2. **A JEPA reader** — perceives the ensemble state at pulse rate
3. **A reflex engine** — fast algorithmic responses (like Pincher reflexes)
4. **An alignment module** — adjusts its own output to match director's feel
5. **A listening module** — hears other instruments and responds

Instruments communicate through the CNS protocol — the same packet-based bus the fleet uses for agent communication. A MIDI note is a packet. A phrase is a pulse. A performance is a session.

## Status

**Concept phase.** Repo created Aug 13, 2026. Design in progress.

## License

MIT

//! CNS Packet definitions — the communication substrate for Fleet Ensemble.
//!
//! Packets are small (≤64 bytes conceptually), unacknowledged, broadcast.
//! This is how nervous systems operate.
//!
//! ## Packet Catalog
//!
//! | ID  | Name                 | Direction          | Frequency      |
//! |-----|----------------------|--------------------|----------------|
//! | 0x01| DIRECTOR_PARAMS      | Director → all     | 10 Hz          |
//! | 0x02| AGENT_INTENT         | Instrument → all   | max 20 Hz      |
//! | 0x03| AGENT_PLAYED         | Instrument → all   | on note emit   |
//! | 0x04| AGENT_DRIFT          | Instrument → all   | 1 Hz           |
//! | 0x05| EMBEDDING_BROADCAST  | Instrument → all   | 2 Hz           |
//! | 0x06| PREDICTION_ERROR     | Instrument → all   | on change      |
//! | 0x07| PHRASE_INTENT        | Instrument → all   | every 2–8s     |
//! | 0x08| ROLE_OFFER           | Instrument → all   | every 2–10s    |
//! | 0x09| ALIGNMENT_REQUEST    | Instrument → inst  | as needed      |
//! | 0x0A| FEEL_TILT            | Director → all     | every pulse    |
//! | 0x0B| INTENT_BROADCAST     | Instrument → all   | on plan change |

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cmp::Ordering;

use super::EMBEDDING_DIM;


/// Priority level for CNS packets.
///
/// The CNS bus uses a single FIFO broadcast channel, which creates a priority
/// inversion risk: a burst of `AgentPlayed` packets (low priority, bursty)
/// can delay `FeelTilt` packets (high priority, timing-critical).
///
/// [`PriorityReceiver`] wraps a `broadcast::Receiver<CnsPacket>` and sorts
/// packets by priority before delivering them to the instrument's main loop.
///
/// Priority assignment:
/// - **Critical**: `FeelTilt` — director timing reference, every pulse
/// - **High**: `AgentPlayed` — reflex triggers need <10ms latency
/// - **Normal**: everything else (embeddings, drift, intents)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PacketPriority {
    Normal = 0,
    High = 1,
    Critical = 2,
}

impl PacketType {
    /// Get the dispatch priority for this packet type.
    ///
    /// See [`PacketPriority`] for rationale.
    pub fn priority(&self) -> PacketPriority {
        match self {
            PacketType::FeelTilt => PacketPriority::Critical,
            PacketType::AgentPlayed => PacketPriority::High,
            _ => PacketPriority::Normal,
        }
    }
}

impl CnsPacket {
    /// Get the priority level for this packet.
    pub fn priority(&self) -> PacketPriority {
        match self {
            CnsPacket::FeelTilt { .. } => PacketPriority::Critical,
            CnsPacket::AgentPlayed { .. } => PacketPriority::High,
            _ => PacketPriority::Normal,
        }
    }

    /// Get the packet type enum for this packet.
    pub fn packet_type(&self) -> PacketType {
        match self {
            CnsPacket::FeelTilt { .. } => PacketType::FeelTilt,
            CnsPacket::IntentBroadcast { .. } => PacketType::IntentBroadcast,
            CnsPacket::AgentPlayed { .. } => PacketType::AgentPlayed,
            CnsPacket::EmbeddingBroadcast { .. } => PacketType::EmbeddingBroadcast,
            CnsPacket::PredictionError { .. } => PacketType::PredictionError,
            CnsPacket::AgentDrift { .. } => PacketType::AgentDrift,
            CnsPacket::PhraseIntent { .. } => PacketType::PhraseIntent,
            CnsPacket::RoleOffer { .. } => PacketType::RoleOffer,
        }
    }
}

/// A priority-ordered wrapper for packets in a receiver's queue.
#[derive(Debug, Clone)]
struct PrioritizedPacket {
    priority: PacketPriority,
    seq: u64, // monotonic sequence for FIFO within same priority
    packet: CnsPacket,
}

impl PartialEq for PrioritizedPacket {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for PrioritizedPacket {}

impl PartialOrd for PrioritizedPacket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedPacket {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first; within same priority, lower seq first (FIFO)
        other.priority.cmp(&self.priority).then(self.seq.cmp(&other.seq))
    }
}

/// Priority-aware receiver that wraps tokio's broadcast receiver.
///
/// Instead of a flat FIFO, this drains available packets from the broadcast
/// channel and delivers them in priority order. This prevents a burst of
/// low-priority packets from blocking timing-critical FeelTilt delivery.
///
/// Usage:
/// ```text
/// let mut prio_rx = PriorityReceiver::new(rx);
/// while let Some(pkt) = prio_rx.recv_ordered().await {
///     // packets arrive highest-priority first
/// }
/// ```
pub struct PriorityReceiver {
    inner: tokio::sync::broadcast::Receiver<CnsPacket>,
    seq_counter: u64,
}

impl PriorityReceiver {
    /// Wrap a broadcast receiver with priority sorting.
    pub fn new(rx: tokio::sync::broadcast::Receiver<CnsPacket>) -> Self {
        Self { inner: rx, seq_counter: 0 }
    }

    /// Drain all currently-buffered packets and return them in priority order.
    ///
    /// This does a non-blocking drain of the broadcast channel's buffer,
    /// sorts by priority, and returns the sorted vec. If the channel is
    /// empty, returns an empty vec.
    pub fn drain_ordered(&mut self) -> Vec<CnsPacket> {
        let mut buffered: Vec<PrioritizedPacket> = Vec::new();

        loop {
            match self.inner.try_recv() {
                Ok(pkt) => {
                    let priority = pkt.priority();
                    let seq = self.seq_counter;
                    self.seq_counter += 1;
                    buffered.push(PrioritizedPacket { priority, seq, packet: pkt });
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => break,
                Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => continue,
            }
        }

        // Sort: highest priority first, then FIFO within same priority
        buffered.sort();

        buffered.into_iter().map(|p| p.packet).collect()
    }

    /// Async receive: wait for at least one packet, then drain and sort.
    ///
    /// Returns None if the channel is closed.
    pub async fn recv_ordered(&mut self) -> Option<Vec<CnsPacket>> {
        // Block on the first packet
        match self.inner.recv().await {
            Ok(first_pkt) => {
                let priority = first_pkt.priority();
                let seq = self.seq_counter;
                self.seq_counter += 1;
                let mut buffered = vec![PrioritizedPacket { priority, seq, packet: first_pkt }];

                // Non-blocking drain of any remaining buffered packets
                loop {
                    match self.inner.try_recv() {
                        Ok(pkt) => {
                            let priority = pkt.priority();
                            let seq = self.seq_counter;
                            self.seq_counter += 1;
                            buffered.push(PrioritizedPacket { priority, seq, packet: pkt });
                        }
                        Err(tokio::sync::broadcast::error::TryRecvError::Empty) => break,
                        Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
                        Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => continue,
                    }
                }

                buffered.sort();
                Some(buffered.into_iter().map(|p| p.packet).collect())
            }
            Err(_) => None,
        }
    }
}

/// Packet type IDs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PacketType {
    DirectorParams = 0x01,
    AgentIntent = 0x02,
    AgentPlayed = 0x03,
    AgentDrift = 0x04,
    EmbeddingBroadcast = 0x05,
    PredictionError = 0x06,
    PhraseIntent = 0x07,
    RoleOffer = 0x08,
    AlignmentRequest = 0x09,
    FeelTilt = 0x0A,
    IntentBroadcast = 0x0B,
}

/// Top-level CNS packet wrapper. All bus messages are this type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CnsPacket {
    /// Director → all: global feel parameters + per-instrument offsets
    FeelTilt {
        timestamp_us: u64,
        seq: u32,
        tilt: FeelTiltPayload,
    },

    /// Instrument → all: "Here is what I'm about to play"
    IntentBroadcast {
        sender_id: u16,
        timestamp_us: u64,
        intent: NoteIntentPayload,
    },

    /// Instrument → all: "I just played this note"
    AgentPlayed {
        sender_id: u16,
        timestamp_us: u64,
        pitch: u8,
        velocity: u8,
    },

    /// Instrument → all: current JEPA embedding
    EmbeddingBroadcast {
        sender_id: u16,
        timestamp_us: u64,
        embedding: Vec<f32>,
    },

    /// Instrument → all: prediction error (surprise signal)
    PredictionError {
        sender_id: u16,
        error: f32,
    },

    /// Instrument → all: clock drift estimate
    AgentDrift {
        sender_id: u16,
        clock_error_us: i32,
    },

    /// Instrument → all: structural-level intent for next phrase
    PhraseIntent {
        sender_id: u16,
        phrase_id: u32,
        contour: u8,
        energy_target: f32,
    },

    /// Instrument → all: role negotiation
    RoleOffer {
        sender_id: u16,
        role: u8,
        confidence: f32,
    },
}

// ─── Payload Types ────────────────────────────────────────────────

/// The 7-dimensional Feel Space parameters broadcast by the Director.
///
/// `F = (ρ, ε, σ, τ, γ, λ, Φ)`
///
/// See: Director Design §2.2 "The Seven Feel Parameters"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeelSpace {
    /// ρ (rho) — Pulse Density [0, 1].
    /// Micro-timing variance. 0 = metronomic, 1 = polyrhythmic chaos.
    pub rho: f32,

    /// ε (epsilon) — Energy Flux [-1, +1].
    /// Rate of change of global dynamic level. Positive = crescendo.
    pub epsilon: f32,

    /// σ (sigma) — Harmonic Tilt [-1, +1].
    /// Pushes toward (+) or away from (-) tonal centroid.
    pub sigma: f32,

    /// τ (tau) — Temporal Asymmetry [0.5, 0.8].
    /// Swing ratio. 0.5 = straight, 0.66 = triplet, 0.8 = deep pocket.
    pub tau: f32,

    /// γ (gamma) — Coupling Pressure [0, 1].
    /// Strength of imitation/alignment. High = flocking, low = individuation.
    pub gamma: f32,

    /// λ (lambda) — Risk Appetite [0, 1].
    /// Stochastic perturbation allowed. High = exploration, low = restraint.
    pub lambda: f32,

    /// Φ (phi) — Articulation (attack, release).
    /// Biases staccato vs. legato. attack > 0 = sharper, release < 0 = shorter.
    pub phi: [f32; 2],
}

impl Default for FeelSpace {
    fn default() -> Self {
        Self {
            rho: 0.3,
            epsilon: 0.0,
            sigma: 0.0,
            tau: 0.5, // straight
            gamma: 0.5,
            lambda: 0.2,
            phi: [0.0, 0.0],
        }
    }
}

impl FeelSpace {
    /// Clamp all parameters to their valid ranges.
    pub fn clamp(&mut self) {
        self.rho = self.rho.clamp(0.0, 1.0);
        self.epsilon = self.epsilon.clamp(-1.0, 1.0);
        self.sigma = self.sigma.clamp(-1.0, 1.0);
        self.tau = self.tau.clamp(0.5, 0.8);
        self.gamma = self.gamma.clamp(0.0, 1.0);
        self.lambda = self.lambda.clamp(0.0, 1.0);
        self.phi[0] = self.phi[0].clamp(-1.0, 1.0);
        self.phi[1] = self.phi[1].clamp(-1.0, 1.0);
    }

    /// Exponential smoothing toward a target feel space.
    /// `Tilt_actual(t) = α × Tilt_target + (1 - α) × Tilt_actual(t-1)`
    pub fn smooth_toward(&mut self, target: &FeelSpace, alpha: f32) {
        self.rho = alpha * target.rho + (1.0 - alpha) * self.rho;
        self.epsilon = alpha * target.epsilon + (1.0 - alpha) * self.epsilon;
        self.sigma = alpha * target.sigma + (1.0 - alpha) * self.sigma;
        self.tau = alpha * target.tau + (1.0 - alpha) * self.tau;
        self.gamma = alpha * target.gamma + (1.0 - alpha) * self.gamma;
        self.lambda = alpha * target.lambda + (1.0 - alpha) * self.lambda;
        self.phi[0] = alpha * target.phi[0] + (1.0 - alpha) * self.phi[0];
        self.phi[1] = alpha * target.phi[1] + (1.0 - alpha) * self.phi[1];
        self.clamp();
    }
}

/// Per-instrument sparse offsets applied on top of the global feel space.
///
/// `Tilt_i = Global_Tilt + Offset_i`
pub type InstrumentOffsets = HashMap<String, FeelSpace>;

/// Emergence flag — tells instruments how to treat detected emergent patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EmergenceFlag {
    #[default]
    None,
    /// Director flattens its tilt — let the emergent pattern breathe.
    Protected,
    /// Director actively bends geometry to amplify the emergent pattern.
    Amplified,
}

/// Full FEEL_TILT payload broadcast every pulse (~125ms).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeelTiltPayload {
    /// The 7 global feel parameters
    pub global: FeelSpace,
    /// Per-instrument sparse deltas (usually empty or small)
    #[serde(default)]
    pub offsets: InstrumentOffsets,
    /// How strongly to apply (0 = whisper, 1 = insist)
    pub confidence: f32,
    /// Emergence status
    pub emergence: EmergenceFlag,
}

impl Default for FeelTiltPayload {
    fn default() -> Self {
        Self {
            global: FeelSpace::default(),
            offsets: HashMap::new(),
            confidence: 0.5,
            emergence: EmergenceFlag::None,
        }
    }
}

/// Note intent — "here's what I'm about to play."
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteIntentPayload {
    /// Original score time in microseconds
    pub nominal_time_us: u64,
    /// MIDI note number
    pub pitch: u8,
    /// Base MIDI velocity (0-127)
    pub velocity: u8,
    /// How certain the instrument is about playing this note (0.0–1.0)
    pub confidence: f32,
    /// Live-adjusted timing offset from nominal (microseconds)
    pub timing_offset_us: i32,
    /// Live-adjusted velocity delta
    pub velocity_bias: i8,
}

/// Stub: encode a NoteIntentPayload to raw bytes for the bus.
/// In production, this would be a tight binary format (postcard / bincode).
pub fn encode_packet(pkt: &CnsPacket) -> Vec<u8> {
    serde_json::to_vec(pkt).unwrap_or_default()
}

/// Decode a CnsPacket from raw bytes.
pub fn decode_packet(data: &[u8]) -> Option<CnsPacket> {
    serde_json::from_slice(data).ok()
}

/// Helper: construct an embedding broadcast packet from a fixed-size array.
pub fn make_embedding_packet(
    sender_id: u16,
    timestamp_us: u64,
    embedding: &[f32; EMBEDDING_DIM],
) -> CnsPacket {
    CnsPacket::EmbeddingBroadcast {
        sender_id,
        timestamp_us,
        embedding: embedding.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feel_space_default_is_straight() {
        let fs = FeelSpace::default();
        assert_eq!(fs.tau, 0.5); // straight, no swing
        assert_eq!(fs.gamma, 0.5); // moderate coupling
    }

    #[test]
    fn feel_space_clamp() {
        let mut fs = FeelSpace {
            rho: -1.0,
            epsilon: 5.0,
            sigma: -2.0,
            tau: 0.0,
            gamma: 10.0,
            lambda: -5.0,
            phi: [10.0, -10.0],
        };
        fs.clamp();
        assert!((0.0..=1.0).contains(&fs.rho));
        assert!((-1.0..=1.0).contains(&fs.epsilon));
        assert!((-1.0..=1.0).contains(&fs.sigma));
        assert!((0.5..=0.8).contains(&fs.tau));
        assert!((0.0..=1.0).contains(&fs.gamma));
        assert!((0.0..=1.0).contains(&fs.lambda));
        assert!((-1.0..=1.0).contains(&fs.phi[0]));
        assert!((-1.0..=1.0).contains(&fs.phi[1]));
    }

    #[test]
    fn feel_space_smooth_toward() {
        let mut current = FeelSpace::default();
        let target = FeelSpace {
            rho: 1.0,
            epsilon: 1.0,
            sigma: 1.0,
            tau: 0.8,
            gamma: 1.0,
            lambda: 1.0,
            phi: [1.0, 1.0],
        };
        current.smooth_toward(&target, 0.5);
        // Each value should be halfway between default and target
        approx::assert_relative_eq!(current.rho, 0.65, epsilon = 1e-6);
        approx::assert_relative_eq!(current.epsilon, 0.5, epsilon = 1e-6);
        approx::assert_relative_eq!(current.gamma, 0.75, epsilon = 1e-6);
        approx::assert_relative_eq!(current.tau, 0.65, epsilon = 1e-6);
    }

    #[test]
    fn feel_tilt_serialization_roundtrip() {
        let tilt = FeelTiltPayload {
            global: FeelSpace {
                rho: 0.4,
                epsilon: 0.15,
                sigma: -0.2,
                tau: 0.62,
                gamma: 0.7,
                lambda: 0.25,
                phi: [0.4, -0.1],
            },
            offsets: HashMap::new(),
            confidence: 0.8,
            emergence: EmergenceFlag::None,
        };
        let json = serde_json::to_string(&tilt).unwrap();
        let decoded: FeelTiltPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(tilt, decoded);
    }

    #[test]
    fn cns_packet_encode_decode_roundtrip() {
        let pkt = CnsPacket::FeelTilt {
            timestamp_us: 1000,
            seq: 1,
            tilt: FeelTiltPayload::default(),
        };
        let encoded = encode_packet(&pkt);
        let decoded = decode_packet(&encoded).unwrap();
        match decoded {
            CnsPacket::FeelTilt { seq, .. } => assert_eq!(seq, 1),
            _ => panic!("wrong packet type"),
        }
    }

    #[test]
    fn emergence_flag_default_is_none() {
        assert_eq!(EmergenceFlag::default(), EmergenceFlag::None);
    }

    // ─── Priority Queueing Tests ──────────────────────────────────

    #[test]
    fn feel_tilt_is_critical_priority() {
        let pkt = CnsPacket::FeelTilt {
            timestamp_us: 0,
            seq: 0,
            tilt: FeelTiltPayload::default(),
        };
        assert_eq!(pkt.priority(), PacketPriority::Critical);
    }

    #[test]
    fn agent_played_is_high_priority() {
        let pkt = CnsPacket::AgentPlayed {
            sender_id: 1,
            timestamp_us: 0,
            pitch: 60,
            velocity: 100,
        };
        assert_eq!(pkt.priority(), PacketPriority::High);
    }

    #[test]
    fn embedding_broadcast_is_normal_priority() {
        let pkt = CnsPacket::EmbeddingBroadcast {
            sender_id: 1,
            timestamp_us: 0,
            embedding: vec![0.0; EMBEDDING_DIM],
        };
        assert_eq!(pkt.priority(), PacketPriority::Normal);
    }

    #[test]
    fn priority_ordering_is_correct() {
        assert!(PacketPriority::Critical > PacketPriority::High);
        assert!(PacketPriority::High > PacketPriority::Normal);
    }

    #[test]
    fn priority_receiver_drains_ordered() {
        let (tx, rx) = tokio::sync::broadcast::channel(64);

        // Send a burst: Normal, then Critical, then High
        let _ = tx.send(CnsPacket::EmbeddingBroadcast {
            sender_id: 1,
            timestamp_us: 0,
            embedding: vec![],
        });
        let _ = tx.send(CnsPacket::FeelTilt {
            timestamp_us: 1,
            seq: 1,
            tilt: FeelTiltPayload::default(),
        });
        let _ = tx.send(CnsPacket::AgentPlayed {
            sender_id: 2,
            timestamp_us: 2,
            pitch: 60,
            velocity: 100,
        });

        let mut prio_rx = PriorityReceiver::new(rx);
        let drained = prio_rx.drain_ordered();

        // Should arrive in priority order: Critical, High, Normal
        assert_eq!(drained.len(), 3);
        assert!(matches!(drained[0], CnsPacket::FeelTilt { .. }));
        assert!(matches!(drained[1], CnsPacket::AgentPlayed { .. }));
        assert!(matches!(drained[2], CnsPacket::EmbeddingBroadcast { .. }));
    }

    #[test]
    fn priority_receiver_preserves_fifo_within_same_priority() {
        let (tx, rx) = tokio::sync::broadcast::channel(64);

        // Send two Normal-priority packets
        let _ = tx.send(CnsPacket::EmbeddingBroadcast {
            sender_id: 1,
            timestamp_us: 100,
            embedding: vec![],
        });
        let _ = tx.send(CnsPacket::EmbeddingBroadcast {
            sender_id: 2,
            timestamp_us: 200,
            embedding: vec![],
        });

        let mut prio_rx = PriorityReceiver::new(rx);
        let drained = prio_rx.drain_ordered();

        assert_eq!(drained.len(), 2);
        // FIFO: sender_id=1 should come first
        match &drained[0] {
            CnsPacket::EmbeddingBroadcast { sender_id, .. } => assert_eq!(*sender_id, 1),
            _ => panic!("wrong packet"),
        }
        match &drained[1] {
            CnsPacket::EmbeddingBroadcast { sender_id, .. } => assert_eq!(*sender_id, 2),
            _ => panic!("wrong packet"),
        }
    }
}

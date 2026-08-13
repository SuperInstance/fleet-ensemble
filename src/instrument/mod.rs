//! Instrument module — individual instrument agents.
//!
//! Each instrument is a self-contained agent with five modules:
//! - [`voice`] — identity (instrument type, range, personality)
//! - [`jepa_reader`] — JEPA perception at 62.5Hz
//! - [`reflex`] — <10ms hard-coded musical responses
//! - [`alignment`] — Kalman-filtered phase-lock loop
//! - [`listening`] — peer attention allocation
//!
//! See: `docs/instrument-agent-design.md`

pub mod alignment;
pub mod jepa_reader;
pub mod listening;
pub mod reflex;
pub mod voice;

use std::time::Duration;

use tokio::sync::broadcast;
use tracing::{debug, info};

use crate::protocol::{CnsPacket, EMBEDDING_DIM};

pub use voice::{Personality, VoiceClass};

const PERCEPTION_INTERVAL: Duration = Duration::from_millis(16); // 62.5 Hz
const PULSE_INTERVAL: Duration = Duration::from_millis(125);

/// An individual instrument agent — one player in the ensemble.
pub struct InstrumentAgent {
    /// Unique instrument ID (maps to MIDI channel).
    pub id: u16,
    /// Voice class (Piano, Bass, Drums, etc.)
    pub voice_class: VoiceClass,
    /// Personality fingerprint.
    pub personality: Personality,
    /// CNS bus sender (for broadcasting packets)
    cns_tx: broadcast::Sender<CnsPacket>,
    /// Current ensemble embedding (JEPA perception)
    ensemble_embedding: [f32; EMBEDDING_DIM],
    /// Current prediction error (surprise signal)
    prediction_error: f32,
    /// Latest director feel-tilt received
    current_tilt: Option<crate::protocol::FeelTiltPayload>,
    /// Alignment state (spring-damper + Kalman)
    alignment: alignment::AlignmentState,
    /// Listening state (peer attention)
    listening: listening::ListeningState,
}

impl InstrumentAgent {
    pub fn new(id: u16, voice_class: VoiceClass, cns_tx: broadcast::Sender<CnsPacket>) -> Self {
        let personality = voice_class.default_personality();
        info!(
            "InstrumentAgent {} created: {:?} (alignment_gain={:.2})",
            id, voice_class, personality.alignment_gain
        );

        let alignment = alignment::AlignmentState::new(&personality);

        Self {
            id,
            voice_class,
            personality,
            cns_tx,
            ensemble_embedding: [0.0; EMBEDDING_DIM],
            prediction_error: 0.0,
            current_tilt: None,
            alignment,
            listening: listening::ListeningState::new(),
        }
    }

    /// Main loop: receive packets, perceive, align, emit.
    pub async fn run(&mut self, mut rx: broadcast::Receiver<CnsPacket>) {
        info!("InstrumentAgent {} ({:?}) running", self.id, self.voice_class);

        let mut perception_timer = tokio::time::interval(PERCEPTION_INTERVAL);
        let mut pulse_timer = tokio::time::interval(PULSE_INTERVAL);

        loop {
            tokio::select! {
                // Drain CNS bus
                Ok(pkt) = rx.recv() => {
                    self.handle_packet(pkt);
                }
                // Perception tick (62.5 Hz)
                _ = perception_timer.tick() => {
                    self.perception_tick();
                }
                // Pulse tick (~125ms) — broadcast embedding
                _ = pulse_timer.tick() => {
                    self.pulse_tick();
                }
            }
        }
    }

    /// Handle an incoming CNS packet.
    fn handle_packet(&mut self, pkt: CnsPacket) {
        match pkt {
            CnsPacket::FeelTilt { tilt, .. } => {
                self.current_tilt = Some(tilt);
            }
            CnsPacket::EmbeddingBroadcast { sender_id, embedding, .. } => {
                // Store peer embedding for perception
                self.listening.update_peer_embedding(sender_id, &embedding);
            }
            CnsPacket::AgentPlayed { sender_id, pitch, velocity, .. } => {
                // Check reflex responses
                let reflex_response = self.voice_class.check_reflex(
                    sender_id,
                    pitch,
                    velocity,
                );
                if let Some(resp) = reflex_response {
                    debug!("Agent {} reflex: {:?}", self.id, resp);
                }
            }
            CnsPacket::PredictionError { sender_id, error, .. } => {
                self.listening.update_peer_error(sender_id, error);
            }
            CnsPacket::AgentDrift { sender_id, clock_error_us, .. } => {
                self.listening.update_peer_drift(sender_id, clock_error_us);
            }
            _ => {}
        }
    }

    /// Perception cycle — runs at 62.5 Hz.
    fn perception_tick(&mut self) {
        // In the stub, the ensemble embedding is a blend of peer embeddings
        // weighted by attention.
        let peer_embeddings = self.listening.get_peer_embeddings();
        if !peer_embeddings.is_empty() {
            // Simple average for the stub
            let mut avg = [0.0f32; EMBEDDING_DIM];
            let count = peer_embeddings.len() as f32;
            for emb in &peer_embeddings {
                for (i, &val) in emb.iter().enumerate() {
                    avg[i] += val;
                }
            }
            for val in &mut avg {
                *val /= count;
            }
            self.ensemble_embedding = avg;
        }

        // Update alignment from current tilt
        if let Some(ref tilt) = self.current_tilt {
            self.alignment.update(tilt, &self.personality);
        }
    }

    /// Pulse tick — broadcast embedding and status.
    fn pulse_tick(&mut self) {
        // Broadcast current embedding
        let pkt = CnsPacket::EmbeddingBroadcast {
            sender_id: self.id,
            timestamp_us: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as u64,
            embedding: self.ensemble_embedding.to_vec(),
        };
        let _ = self.cns_tx.send(pkt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_agent(id: u16, voice: VoiceClass) -> (InstrumentAgent, broadcast::Receiver<CnsPacket>) {
        let (tx, rx) = broadcast::channel(64);
        let agent = InstrumentAgent::new(id, voice, tx);
        (agent, rx)
    }

    #[test]
    fn piano_has_soft_alignment() {
        let (agent, _) = make_agent(1, VoiceClass::Piano);
        assert!(agent.personality.alignment_gain < 0.5);
    }

    #[test]
    fn bass_has_strong_alignment() {
        let (agent, _) = make_agent(2, VoiceClass::Bass);
        assert!(agent.personality.alignment_gain > 0.5);
    }

    #[test]
    fn drums_have_absolute_alignment() {
        let (agent, _) = make_agent(3, VoiceClass::Drums);
        assert!(agent.personality.alignment_gain > 0.8);
    }

    #[test]
    fn agent_starts_with_no_tilt() {
        let (agent, _) = make_agent(1, VoiceClass::Piano);
        assert!(agent.current_tilt.is_none());
    }

    #[test]
    fn agent_starts_with_zero_prediction_error() {
        let (agent, _) = make_agent(1, VoiceClass::Piano);
        assert_eq!(agent.prediction_error, 0.0);
    }
}

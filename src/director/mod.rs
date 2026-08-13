//! Director module — the ensemble-level intelligence.
//!
//! The Director is a tri-chamber mind (Oracle, Maestro, Pulse) that:
//! 1. Perceives the ensemble through JEPA embedding point clouds
//! 2. Computes a 5-level perceptual stack (centroid, dispersion, velocity, flux, coherence)
//! 3. Outputs a 7-dimensional Feel Space tilt every pulse
//! 4. Detects and amplifies emergence
//!
//! See: `docs/director-design.md`

pub mod emergence;
pub mod feel_space;
pub mod modes;
pub mod perception;

use std::sync::Arc;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use tokio::sync::broadcast;
use tracing::{debug, info};

use crate::protocol::{
    CnsPacket, EMBEDDING_DIM, EmergenceFlag, FeelSpace, FeelTiltPayload,
};

const PULSE_INTERVAL: Duration = Duration::from_millis(125);

/// The Agentic Director — perceives the ensemble and broadcasts feel-tilt.
///
/// This is the stub implementation: it reads instrument embeddings from
/// a shared registry, computes the perceptual stack, and outputs a
/// FeelTilt packet every pulse.
pub struct Director {
    /// Sender for the CNS broadcast bus
    cns_tx: broadcast::Sender<CnsPacket>,
    /// Shared registry of latest embeddings per instrument
    embeddings: Arc<DashMap<u16, [f32; EMBEDDING_DIM]>>,
    /// Current perceptual state
    perception: perception::PerceptionState,
    /// Current feel space (smoothed)
    feel: FeelSpace,
    /// Operational mode
    mode: modes::DirectorMode,
    /// Sequence counter for FeelTilt packets
    seq: u32,
}

impl Director {
    pub fn new(
        cns_tx: broadcast::Sender<CnsPacket>,
        embeddings: Arc<DashMap<u16, [f32; EMBEDDING_DIM]>>,
    ) -> Self {
        Self {
            cns_tx,
            embeddings,
            perception: perception::PerceptionState::default(),
            feel: FeelSpace::default(),
            mode: modes::DirectorMode::JazzBandleader,
            seq: 0,
        }
    }

    /// Set the operational mode.
    pub fn set_mode(&mut self, mode: modes::DirectorMode) {
        info!("Director mode → {:?}", mode);
        self.mode = mode;
    }

    /// Main loop: perceive → compute tilt → broadcast, every 125ms.
    pub async fn run(mut self) {
        let mut interval = tokio::time::interval(PULSE_INTERVAL);
        info!("Director running in {:?} mode", self.mode);

        loop {
            interval.tick().await;
            self.pulse();
        }
    }

    /// Process one pulse cycle.
    fn pulse(&mut self) {
        // 1. Gather embeddings from all instruments
        let point_cloud: Vec<[f32; EMBEDDING_DIM]> = self
            .embeddings
            .iter()
            .map(|entry| *entry.value())
            .collect();

        if point_cloud.is_empty() {
            debug!("Director pulse: no embeddings yet");
            return;
        }

        // 2. Update perceptual stack
        self.perception.update(&point_cloud);

        // 3. Compute target feel from mode + perception
        let target = self.mode.compute_feel(&self.perception);

        // 4. Smooth toward target (exponential smoothing, α from mode)
        let alpha = self.mode.smoothing_alpha();
        self.feel.smooth_toward(&target, alpha);

        // 5. Check for emergence
        let emergence = self.perception.detect_emergence();

        // 6. Build and broadcast FeelTilt packet
        let tilt = FeelTiltPayload {
            global: self.feel.clone(),
            offsets: Default::default(),
            confidence: self.mode.confidence(),
            emergence,
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        let pkt = CnsPacket::FeelTilt {
            timestamp_us: timestamp,
            seq: self.seq,
            tilt,
        };

        // Best-effort send — if no subscribers, that's fine
        let _ = self.cns_tx.send(pkt);
        self.seq += 1;

        debug!(
            "Director pulse {}: dispersion={:.3}, coherence={:.3}",
            self.seq, self.perception.dispersion, self.perception.coherence
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_director() -> (Director, broadcast::Receiver<CnsPacket>) {
        let (tx, rx) = broadcast::channel(64);
        let embeddings = Arc::new(DashMap::new());
        let director = Director::new(tx, embeddings);
        (director, rx)
    }

    #[test]
    fn director_starts_in_default_mode() {
        let (director, _) = make_director();
        assert_eq!(director.mode, modes::DirectorMode::JazzBandleader);
    }

    #[test]
    fn director_starts_with_zero_seq() {
        let (director, _) = make_director();
        assert_eq!(director.seq, 0);
    }

    #[test]
    fn feel_space_starts_default() {
        let (director, _) = make_director();
        assert_eq!(director.feel.tau, 0.5); // straight
    }
}

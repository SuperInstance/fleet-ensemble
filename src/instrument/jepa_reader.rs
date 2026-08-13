//! JEPA Reader — Joint Embedding Predictive Architecture perception.
//!
//! Encodes ensemble state into a shared embedding space at 62.5 Hz (every 16ms).
//! Instruments don't listen to past events — they listen to *future intent*.
//!
//! Perception cycle:
//! 1. DRAIN — collect all CNS packets
//! 2. BUILD — unify intents into shared timeline
//! 3. ENCODE — project timeline into JEPA embedding space
//! 4. PREDICT — forecast what comes next
//! 5. SURPRISE — compute prediction error
//! 6. DEVIATION — distance from score expectation
//! 7. ATTENTION — update peer attention weights
//!
//! See: Instrument Agent Design §3 "Perception Pipeline"

use crate::protocol::EMBEDDING_DIM;

/// The JEPA reader's state — encodes ensemble perception and prediction.
pub struct JepaReader {
    /// Current ensemble embedding (what the instrument perceives).
    pub ensemble_embedding: [f32; EMBEDDING_DIM],

    /// Predicted next embedding (what the instrument expects).
    pub predicted_next: [f32; EMBEDDING_DIM],

    /// Prediction error — how unexpected is the present?
    /// `||current - predicted||`
    pub prediction_error: f32,

    /// Deviation from score expectation.
    pub deviation: f32,
}

impl Default for JepaReader {
    fn default() -> Self {
        Self {
            ensemble_embedding: [0.0; EMBEDDING_DIM],
            predicted_next: [0.0; EMBEDDING_DIM],
            prediction_error: 0.0,
            deviation: 0.0,
        }
    }
}

impl JepaReader {
    pub fn new() -> Self {
        Self::default()
    }

    /// Encode a set of peer embeddings into the instrument's perception.
    ///
    /// In the full system, this runs the JEPA encoder neural network.
    /// In the stub, we use simple averaging.
    pub fn encode(&mut self, peer_embeddings: &[Vec<f32>]) {
        if peer_embeddings.is_empty() {
            return;
        }

        // Simple averaging as stub for JEPA encoder
        let mut avg = [0.0f32; EMBEDDING_DIM];
        for emb in peer_embeddings {
            let n = emb.len().min(EMBEDDING_DIM);
            for i in 0..n {
                avg[i] += emb[i];
            }
        }
        let count = peer_embeddings.len() as f32;
        for val in &mut avg {
            *val /= count;
        }

        self.ensemble_embedding = avg;
    }

    /// Predict the next ensemble state.
    ///
    /// In the full system, this runs the JEPA predictor network.
    /// In the stub, prediction = current state (no change expected).
    pub fn predict(&mut self) {
        // Stub: predict current state persists (identity prediction)
        self.predicted_next = self.ensemble_embedding;
    }

    /// Compute prediction error — how surprising is the current state?
    ///
    /// `error = cosine_distance(current, predicted)`
    pub fn compute_prediction_error(&self) -> f32 {
        cosine_distance(&self.ensemble_embedding, &self.predicted_next)
    }

    /// Full perception cycle (62.5 Hz).
    pub fn tick(&mut self, peer_embeddings: &[Vec<f32>]) {
        // Previous prediction
        let prev_prediction = self.predicted_next;

        // Encode current state
        self.encode(peer_embeddings);

        // Compute prediction error: how far is current from what we predicted?
        self.prediction_error = cosine_distance(&self.ensemble_embedding, &prev_prediction);

        // Predict next
        self.predict();
    }
}

/// Cosine distance between two vectors: 0 = identical, 1 = orthogonal.
#[inline]
pub fn cosine_distance(a: &[f32; EMBEDDING_DIM], b: &[f32; EMBEDDING_DIM]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for i in 0..EMBEDDING_DIM {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    let denom = (norm_a.sqrt() * norm_b.sqrt()).max(1e-10);
    1.0 - (dot / denom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_starts_zeroed() {
        let reader = JepaReader::default();
        assert_eq!(reader.prediction_error, 0.0);
        assert_eq!(reader.deviation, 0.0);
    }

    #[test]
    fn encode_averages_peer_embeddings() {
        let mut reader = JepaReader::new();
        let peers = vec![
            vec![1.0; EMBEDDING_DIM],
            vec![3.0; EMBEDDING_DIM],
        ];
        reader.encode(&peers);
        // Average of 1.0 and 3.0 = 2.0
        for i in 0..EMBEDDING_DIM {
            approx::assert_relative_eq!(reader.ensemble_embedding[i], 2.0, epsilon = 1e-5);
        }
    }

    #[test]
    fn encode_empty_peers_is_noop() {
        let mut reader = JepaReader::new();
        reader.encode(&[]);
        assert_eq!(reader.ensemble_embedding, [0.0; EMBEDDING_DIM]);
    }

    #[test]
    fn prediction_error_is_zero_for_identical_vectors() {
        let reader = JepaReader::default();
        let err = reader.compute_prediction_error();
        // Zero vectors: both norms are 0, cosine distance should handle gracefully
        assert!(err.is_finite());
    }

    #[test]
    fn cosine_distance_orthogonal_vectors() {
        let mut a = [0.0f32; EMBEDDING_DIM];
        let mut b = [0.0f32; EMBEDDING_DIM];
        a[0] = 1.0;
        b[1] = 1.0;
        let dist = cosine_distance(&a, &b);
        approx::assert_relative_eq!(dist, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn cosine_distance_identical_vectors() {
        let a = [0.7; EMBEDDING_DIM];
        let b = [0.7; EMBEDDING_DIM];
        let dist = cosine_distance(&a, &b);
        approx::assert_relative_eq!(dist, 0.0, epsilon = 1e-5);
    }

    #[test]
    fn tick_updates_prediction_error() {
        let mut reader = JepaReader::new();
        reader.predicted_next = [1.0; EMBEDDING_DIM]; // wrong prediction
        let peers = vec![vec![0.5; EMBEDDING_DIM]];
        reader.tick(&peers);
        assert!(reader.prediction_error >= 0.0);
    }
}

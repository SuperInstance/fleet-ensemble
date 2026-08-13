//! Listening Module — peer attention allocation.
//!
//! Each instrument has a limited attention budget. It decides which peers
//! to listen to most, based on:
//! - Director weight parameter (which instrument should carry the moment)
//! - Current role (lead, support, padding, tacet)
//! - Prediction error signals (surprising peers get attention)
//!
//! See: Instrument Agent Design §3 "Perception Pipeline" (step 7: attention)

use std::collections::HashMap;

use crate::protocol::EMBEDDING_DIM;

/// Maximum number of peers an instrument can track.
const MAX_PEERS: usize = 31;

/// Listening state — tracks peer embeddings, errors, and attention weights.
pub struct ListeningState {
    /// Latest embedding per peer (instrument ID → embedding).
    peer_embeddings: HashMap<u16, [f32; EMBEDDING_DIM]>,

    /// Latest prediction error per peer.
    peer_errors: HashMap<u16, f32>,

    /// Clock drift estimate per peer (microseconds).
    peer_drift: HashMap<u16, i32>,

    /// Attention weight per peer [0, 1].
    attention_weights: HashMap<u16, f32>,

    /// Current role of this instrument.
    current_role: Role,
}

/// Musical role — determines attention allocation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Role {
    /// Carrying the melody / main line.
    Lead,
    /// Accompanying — comping, rhythmic support.
    #[default]
    Support,
    /// Filling space — pads, held notes.
    Padding,
    /// Soloing — has the floor.
    Solo,
    /// Silent — not playing.
    Tacet,
}

impl ListeningState {
    pub fn new() -> Self {
        Self {
            peer_embeddings: HashMap::new(),
            peer_errors: HashMap::new(),
            peer_drift: HashMap::new(),
            attention_weights: HashMap::new(),
            current_role: Role::Support,
        }
    }

    /// Update a peer's latest embedding.
    pub fn update_peer_embedding(&mut self, peer_id: u16, embedding: &[f32]) {
        let mut emb = [0.0f32; EMBEDDING_DIM];
        let n = embedding.len().min(EMBEDDING_DIM);
        emb[..n].copy_from_slice(&embedding[..n]);
        self.peer_embeddings.insert(peer_id, emb);

        // Recompute attention weights
        self.update_attention();
    }

    /// Update a peer's prediction error.
    pub fn update_peer_error(&mut self, peer_id: u16, error: f32) {
        self.peer_errors.insert(peer_id, error);

        // High prediction error triggers immediate attention
        if error > 0.3 {
            self.attention_weights.insert(peer_id, 1.0);
        }
    }

    /// Update a peer's clock drift estimate.
    pub fn update_peer_drift(&mut self, peer_id: u16, drift_us: i32) {
        self.peer_drift.insert(peer_id, drift_us);
    }

    /// Get all current peer embeddings.
    pub fn get_peer_embeddings(&self) -> Vec<&[f32; EMBEDDING_DIM]> {
        self.peer_embeddings.values().collect()
    }

    /// Get attention weight for a specific peer.
    pub fn attention_for(&self, peer_id: u16) -> f32 {
        *self.attention_weights.get(&peer_id).unwrap_or(&0.0)
    }

    /// Set the current role — affects attention allocation.
    pub fn set_role(&mut self, role: Role) {
        self.current_role = role;
        self.update_attention();
    }

    /// Recompute attention weights based on role, errors, and director weight.
    fn update_attention(&mut self) {
        let n_peers = self.peer_embeddings.len();
        if n_peers == 0 {
            return;
        }

        // Base attention: equal distribution
        let base = match self.current_role {
            Role::Lead | Role::Solo => 0.2, // soloists listen less
            Role::Support => 0.5,           // accompanists listen more
            Role::Padding => 0.3,           // pad players listen moderately
            Role::Tacet => 0.8,             // silent players listen most
        };

        // Distribute base attention equally among peers
        let equal_share = base / n_peers as f32;
        for peer_id in self.peer_embeddings.keys() {
            let error_boost = self.peer_errors.get(peer_id).copied().unwrap_or(0.0);
            // Attention = base + error boost
            let weight = (equal_share + error_boost * 0.5).min(1.0);
            self.attention_weights.insert(*peer_id, weight);
        }
    }
}

impl Default for ListeningState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_is_empty() {
        let state = ListeningState::new();
        assert!(state.peer_embeddings.is_empty());
        assert!(state.attention_weights.is_empty());
    }

    #[test]
    fn update_peer_embedding_stores_it() {
        let mut state = ListeningState::new();
        let emb = vec![0.5; EMBEDDING_DIM];
        state.update_peer_embedding(2, &emb);
        assert!(state.peer_embeddings.contains_key(&2));
    }

    #[test]
    fn high_prediction_error_boosts_attention() {
        let mut state = ListeningState::new();
        let emb = vec![0.5; EMBEDDING_DIM];
        state.update_peer_embedding(2, &emb);
        state.update_peer_error(2, 0.8); // high surprise

        let attention = state.attention_for(2);
        assert!(attention > 0.5, "high prediction error should boost attention");
    }

    #[test]
    fn low_prediction_error_keeps_base_attention() {
        let mut state = ListeningState::new();
        let emb = vec![0.5; EMBEDDING_DIM];
        state.update_peer_embedding(2, &emb);
        state.update_peer_error(2, 0.01); // low surprise

        let attention = state.attention_for(2);
        assert!(attention < 0.7, "low error should not boost attention much");
    }

    #[test]
    fn solo_role_listens_less_than_support() {
        let mut solo_state = ListeningState::new();
        solo_state.set_role(Role::Solo);
        solo_state.update_peer_embedding(1, &vec![0.5; EMBEDDING_DIM]);
        solo_state.update_peer_embedding(2, &vec![0.5; EMBEDDING_DIM]);
        let solo_attention = solo_state.attention_for(1);

        let mut support_state = ListeningState::new();
        support_state.set_role(Role::Support);
        support_state.update_peer_embedding(1, &vec![0.5; EMBEDDING_DIM]);
        support_state.update_peer_embedding(2, &vec![0.5; EMBEDDING_DIM]);
        let support_attention = support_state.attention_for(1);

        assert!(solo_attention < support_attention,
            "soloist should listen less than accompanist");
    }

    #[test]
    fn update_peer_drift_stores_it() {
        let mut state = ListeningState::new();
        state.update_peer_drift(3, -500);
        assert_eq!(state.peer_drift.get(&3), Some(&-500));
    }

    #[test]
    fn get_peer_embeddings_returns_all() {
        let mut state = ListeningState::new();
        state.update_peer_embedding(1, &vec![0.1; EMBEDDING_DIM]);
        state.update_peer_embedding(2, &vec![0.2; EMBEDDING_DIM]);
        state.update_peer_embedding(3, &vec![0.3; EMBEDDING_DIM]);
        assert_eq!(state.get_peer_embeddings().len(), 3);
    }

    #[test]
    fn default_role_is_support() {
        let state = ListeningState::new();
        assert_eq!(state.current_role, Role::Support);
    }
}

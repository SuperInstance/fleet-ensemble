//! Perception stack — the Director's 5-level perceptual model.
//!
//! Updated every pulse (~125ms) from instrument embedding point clouds.
//!
//! | Level         | Symbol  | Computation                              |
//! |---------------|---------|------------------------------------------|
//! | Centroid      | C(t)    | Mean of all instrument embeddings        |
//! | Dispersion    | D(t)    | Mean distance from centroid              |
//! | Velocity      | ΔC(t)   | Centroid change since last pulse         |
//! | Rotational    | Ω(t)    | Inner product of positions and velocities|
//! | Coherence     | K(t)    | Fourier stability over 32-pulse window   |
//!
//! See: Director Design §1.2 "The Perceptual Stack"

use super::feel_space::FeelSpaceExt;
use crate::protocol::{EMBEDDING_DIM, EmergenceFlag};

/// Number of pulses to retain for temporal analysis (32 pulses ≈ 4 seconds).
const HISTORY_WINDOW: usize = 32;

/// The Director's perceptual state — five statistics + history.
#[derive(Debug, Clone)]
pub struct PerceptionState {
    /// C(t): centroid of the ensemble embedding point cloud.
    pub centroid: [f32; EMBEDDING_DIM],

    /// D(t): mean distance of instruments from the centroid.
    /// Low = locked-in/unified, high = chaotic/soloistic.
    pub dispersion: f32,

    /// ΔC(t): centroid change since last pulse.
    /// Shows direction the music is heading.
    pub velocity: [f32; EMBEDDING_DIM],

    /// Ω(t): rotational flux — are instruments orbiting or converging?
    pub rotational_flux: f32,

    /// K(t): temporal coherence — groove stability over 32-pulse window.
    /// Fourier stability of centroid trajectory.
    pub coherence: f32,

    /// Previous centroid (for computing velocity).
    prev_centroid: Option<[f32; EMBEDDING_DIM]>,

    /// Centroid history for coherence computation (ring buffer).
    centroid_history: Vec<[f32; EMBEDDING_DIM]>,

    /// Previous per-instrument positions (for rotational flux).
    prev_positions: Vec<[f32; EMBEDDING_DIM]>,
}

impl Default for PerceptionState {
    fn default() -> Self {
        Self {
            centroid: [0.0; EMBEDDING_DIM],
            dispersion: 0.0,
            velocity: [0.0; EMBEDDING_DIM],
            rotational_flux: 0.0,
            coherence: 0.0,
            prev_centroid: None,
            centroid_history: Vec::with_capacity(HISTORY_WINDOW),
            prev_positions: Vec::new(),
        }
    }
}

impl PerceptionState {
    /// Update the perceptual stack from the current point cloud.
    ///
    /// `X(t) = { v_1(t), v_2(t), ..., v_N(t) }`
    pub fn update(&mut self, point_cloud: &[[f32; EMBEDDING_DIM]]) {
        let n = point_cloud.len();
        if n == 0 {
            return;
        }

        // ─── Centroid: C(t) = (1/N) Σ v_i(t) ───────────────────
        let mut new_centroid = [0.0f32; EMBEDDING_DIM];
        for vec in point_cloud {
            for (i, &val) in vec.iter().enumerate() {
                new_centroid[i] += val;
            }
        }
        for val in &mut new_centroid {
            *val /= n as f32;
        }

        // ─── Dispersion: D(t) = (1/N) Σ ‖v_i(t) - C(t)‖ ─────────
        let mut total_dist = 0.0f32;
        for vec in point_cloud {
            total_dist += euclidean_dist(vec, &new_centroid);
        }
        self.dispersion = total_dist / n as f32;

        // ─── Velocity: ΔC(t) = C(t) - C(t-1) ───────────────────
        if let Some(ref prev) = self.prev_centroid {
            for i in 0..EMBEDDING_DIM {
                self.velocity[i] = new_centroid[i] - prev[i];
            }
        }

        // ─── Rotational Flux: Ω(t) = Σ ⟨v_i - C, Δ(v_i - C)⟩ ──
        // Measures whether instruments are orbiting a shared idea
        // (high flux = creative tension) or converging (low flux).
        if !self.prev_positions.is_empty() && self.prev_positions.len() == point_cloud.len() {
            let prev_c = self.prev_centroid.unwrap_or(self.centroid);
            let mut flux = 0.0f32;
            for (i, vec) in point_cloud.iter().enumerate() {
                let prev_vec = &self.prev_positions[i];
                // Position relative to current and previous centroid
                let mut rel_curr = [0.0f32; EMBEDDING_DIM];
                let mut rel_prev = [0.0f32; EMBEDDING_DIM];
                for j in 0..EMBEDDING_DIM {
                    rel_curr[j] = vec[j] - new_centroid[j];
                    rel_prev[j] = prev_vec[j] - prev_c[j];
                }
                // Inner product of current position and velocity
                for j in 0..EMBEDDING_DIM {
                    flux += rel_curr[j] * (rel_curr[j] - rel_prev[j]);
                }
            }
            self.rotational_flux = flux / n as f32;
        }

        // ─── Temporal Coherence: K(t) ──────────────────────────
        // Fourier stability: variance of centroid magnitude over window.
        // Simplified: 1 - normalized_variance of centroid norm.
        self.centroid_history.push(new_centroid);
        if self.centroid_history.len() > HISTORY_WINDOW {
            self.centroid_history.remove(0);
        }
        self.coherence = compute_coherence(&self.centroid_history);

        // ─── Store state ────────────────────────────────────────
        self.prev_positions = point_cloud.to_vec();
        self.prev_centroid = Some(new_centroid);
        self.centroid = new_centroid;
    }

    /// Quick emergence check — stub for the full transfer-entropy +
    /// persistent-homology detector.
    pub fn detect_emergence(&self) -> EmergenceFlag {
        // Simplified: if dispersion is dropping while coherence is high,
        // instruments are converging — flag as potential emergence.
        if self.dispersion < 0.1 && self.coherence > 0.8 {
            EmergenceFlag::Protected
        } else {
            EmergenceFlag::None
        }
    }

    /// Get the current perceptual summary as a flat vector.
    /// Used for feeding into the (future) SSM latent state.
    pub fn summary(&self) -> Vec<f32> {
        let mut v = Vec::with_capacity(EMBEDDING_DIM * 2 + 3);
        v.extend_from_slice(&self.centroid);
        v.extend_from_slice(&self.velocity);
        v.push(self.dispersion);
        v.push(self.rotational_flux);
        v.push(self.coherence);
        v
    }
}

// ─── Helpers ─────────────────────────────────────────────────────

#[inline]
fn euclidean_dist(a: &[f32; EMBEDDING_DIM], b: &[f32; EMBEDDING_DIM]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..EMBEDDING_DIM {
        let d = a[i] - b[i];
        sum += d * d;
    }
    sum.sqrt()
}

/// Compute temporal coherence from centroid history.
///
/// Returns a value in [0, 1] where 1 = very stable groove.
/// Simplified: uses inverse of normalized magnitude variance.
fn compute_coherence(history: &[[f32; EMBEDDING_DIM]]) -> f32 {
    if history.len() < 4 {
        return 0.5; // not enough data — neutral
    }

    // Compute magnitudes
    let mags: Vec<f32> = history.iter().map(|v| vector_magnitude(v)).collect();

    // Variance of magnitudes
    let mean = mags.iter().sum::<f32>() / mags.len() as f32;
    let variance =
        mags.iter().map(|m| (m - mean).powi(2)).sum::<f32>() / mags.len() as f32;

    // Normalize: coherence = 1 / (1 + variance)
    1.0 / (1.0 + variance)
}

#[inline]
fn vector_magnitude(v: &[f32; EMBEDDING_DIM]) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_point_cloud_is_noop() {
        let mut state = PerceptionState::default();
        state.update(&[]);
        assert_eq!(state.dispersion, 0.0);
    }

    #[test]
    fn single_instrument_has_zero_dispersion() {
        let mut state = PerceptionState::default();
        let cloud = vec![[0.5; EMBEDDING_DIM]];
        state.update(&cloud);
        assert_eq!(state.dispersion, 0.0, "single instrument = zero dispersion");
    }

    #[test]
    fn identical_instruments_have_zero_dispersion() {
        let mut state = PerceptionState::default();
        let cloud = vec![
            [0.5; EMBEDDING_DIM],
            [0.5; EMBEDDING_DIM],
            [0.5; EMBEDDING_DIM],
        ];
        state.update(&cloud);
        assert_eq!(state.dispersion, 0.0);
    }

    #[test]
    fn dispersed_instruments_have_positive_dispersion() {
        let mut state = PerceptionState::default();
        let mut cloud = vec![[0.0; EMBEDDING_DIM]; 2];
        // Make them different
        for i in 0..EMBEDDING_DIM {
            cloud[1][i] = 1.0;
        }
        state.update(&cloud);
        assert!(state.dispersion > 0.0);
    }

    #[test]
    fn velocity_is_zero_on_first_update() {
        let mut state = PerceptionState::default();
        let cloud = vec![[0.5; EMBEDDING_DIM]];
        state.update(&cloud);
        assert_eq!(state.velocity, [0.0; EMBEDDING_DIM]);
    }

    #[test]
    fn velocity_updates_on_second_pulse() {
        let mut state = PerceptionState::default();
        let cloud1 = vec![[0.5; EMBEDDING_DIM]];
        state.update(&cloud1);

        let mut cloud2 = vec![[0.6; EMBEDDING_DIM]];
        state.update(&cloud2);

        // velocity should be 0.6 - 0.5 = 0.1 in every dimension
        for i in 0..EMBEDDING_DIM {
            approx::assert_relative_eq!(state.velocity[i], 0.1, epsilon = 1e-5);
        }
    }

    #[test]
    fn coherence_starts_neutral() {
        let state = PerceptionState::default();
        assert_eq!(state.coherence, 0.5);
    }

    #[test]
    fn detect_emergence_returns_none_by_default() {
        let state = PerceptionState::default();
        assert_eq!(state.detect_emergence(), EmergenceFlag::None);
    }

    #[test]
    fn summary_has_correct_length() {
        let state = PerceptionState::default();
        let s = state.summary();
        assert_eq!(s.len(), EMBEDDING_DIM * 2 + 3);
    }
}

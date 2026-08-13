//! Emergence detection — when the music surprises the Director.
//!
//! Two complementary detection signals:
//! 1. **Transfer Entropy Spike** — pairwise info flow between instruments
//! 2. **Topological Persistence** — Betti-1 features in embedding point cloud
//!
//! When emergence is detected and validated (persists > 8 pulses, compatible
//! with the narrative arc), the Director enters the amplification protocol:
//! detect → validate → approve → protect → amplify → nurture → release.
//!
//! See: Director Design §5 "Emergence"

use crate::protocol::EMBEDDING_DIM;

/// Number of recent pulses to retain for transfer entropy computation.
const TE_WINDOW: usize = 16;

/// Minimum number of samples required for statistically meaningful transfer entropy.
///
/// Transfer entropy estimation requires estimating conditional entropies via
/// binning or k-NN methods. With fewer than ~128 samples, the estimator variance
/// exceeds the signal. Below this threshold we return 0.0 (no detected flow).
/// See: Schreiber, Phys. Rev. Lett. 85, 461 (2000); Lindner et al., PLoS ONE 6, e14747 (2011).
const TE_MIN_SAMPLES: usize = 128;

/// Minimum persistence (in pulses) for a topological feature to count as emergence.
const MIN_PERSISTENCE: usize = 8;

/// Stub: compute transfer entropy between two embedding time series.
///
/// `TE(A→B) = H(B_t | B_{t-1:t-k}) - H(B_t | B_{t-1:t-k}, A_{t-1:t-k})`
///
/// Returns how much knowing A's recent history improves prediction of B.
/// In the stub, this uses a simplified Granger-style linear approximation.
///
/// **Minimum sample requirement:** TE estimation requires ≥ `TE_MIN_SAMPLES`
/// (128) samples for statistically meaningful results. With fewer samples,
/// estimator variance exceeds the signal (see Lindner et al., PLoS ONE, 2011).
/// This function returns 0.0 gracefully when insufficient data is available.
pub fn transfer_entropy(
    history_a: &[[f32; EMBEDDING_DIM]],
    history_b: &[[f32; EMBEDDING_DIM]],
) -> f32 {
    let n = history_a.len().min(history_b.len());

    // Graceful fallback: insufficient data for meaningful TE estimation.
    // TE estimation via k-NN or binning needs O(100+) samples; we use 128
    // as the minimum to keep estimator variance below the expected signal.
    // Callers should collect more history before relying on TE results.
    if n < TE_MIN_SAMPLES {
        return 0.0;
    }

    // Simplified: correlation of A's past with B's present, minus B's autocorrelation.
    // This is a crude approximation of transfer entropy.
    let mut te_sum = 0.0f32;
    let lag = 1;

    for t in (lag + 1)..n {
        // B's change from t-1 to t
        let b_delta = vector_norm_diff(&history_b[t], &history_b[t - 1]);
        // A's value at t-lag (the "causal" signal)
        let a_val = vector_magnitude(&history_a[t - lag]);

        // Crude cross-correlation contribution
        te_sum += a_val * b_delta;
    }

    // Normalize
    te_sum / (n as f32)
}

/// Detect if any instrument pairs show emergence (high transfer entropy).
///
/// Returns pairs (i, j) where TE(i→j) exceeds the rolling baseline.
pub fn detect_te_spikes(
    histories: &[Vec<[f32; EMBEDDING_DIM]>],
) -> Vec<(usize, usize, f32)> {
    let n = histories.len();
    let mut spikes = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let te = transfer_entropy(&histories[i], &histories[j]);
            if te > 0.5 {
                spikes.push((i, j, te));
            }
        }
    }

    spikes
}

/// Detect persistent topological features (Betti-1 loops) in the point cloud.
///
/// This is a stub — real persistent homology requires a library like `ripser`
/// or `phat`. Here we approximate by checking if the point cloud forms
/// a stable cluster configuration over time.
pub fn detect_persistent_clusters(
    point_cloud_history: &[Vec<[f32; EMBEDDING_DIM]>],
) -> bool {
    if point_cloud_history.len() < MIN_PERSISTENCE {
        return false;
    }

    // Simplified: check if the centroid positions have been stable
    // (low variance) for the last MIN_PERSISTENCE pulses.
    let recent = &point_cloud_history[point_cloud_history.len() - MIN_PERSISTENCE..];

    // Check that all recent point clouds have similar dispersion
    let mut dispersions = Vec::new();
    for cloud in recent {
        if cloud.is_empty() {
            continue;
        }
        let centroid = compute_centroid(cloud);
        let disp = mean_distance(cloud, &centroid);
        dispersions.push(disp);
    }

    if dispersions.len() < MIN_PERSISTENCE {
        return false;
    }

    // Low variance in dispersion = stable structure
    let mean = dispersions.iter().sum::<f32>() / dispersions.len() as f32;
    let variance = dispersions
        .iter()
        .map(|d| (d - mean).powi(2))
        .sum::<f32>()
        / dispersions.len() as f32;

    variance < 0.01 * mean.max(0.001)
}

/// Full emergence check: combines TE spikes + topological persistence.
pub fn check_emergence(
    histories: &[Vec<[f32; EMBEDDING_DIM]>],
    point_cloud_history: &[Vec<[f32; EMBEDDING_DIM]>],
) -> EmergenceDetection {
    let te_spikes = detect_te_spikes(histories);
    let has_persistent_clusters = detect_persistent_clusters(point_cloud_history);

    let significant_te = te_spikes.iter().any(|(_, _, te)| *te > 0.7);

    match (significant_te, has_persistent_clusters) {
        (true, true) => EmergenceDetection::Strong,
        (true, false) => EmergenceDetection::Weak,
        (false, true) => EmergenceDetection::Weak,
        (false, false) => EmergenceDetection::None,
    }
}

/// Result of emergence detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergenceDetection {
    /// No emergence signals detected.
    None,
    /// Some signals present but not yet validated.
    Weak,
    /// Both transfer entropy and topological persistence confirm emergence.
    Strong,
}

// ─── Helpers ─────────────────────────────────────────────────────

#[inline]
fn vector_magnitude(v: &[f32; EMBEDDING_DIM]) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}

#[inline]
fn vector_norm_diff(a: &[f32; EMBEDDING_DIM], b: &[f32; EMBEDDING_DIM]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..EMBEDDING_DIM {
        let d = a[i] - b[i];
        sum += d * d;
    }
    sum.sqrt()
}

fn compute_centroid(cloud: &[[f32; EMBEDDING_DIM]]) -> [f32; EMBEDDING_DIM] {
    let mut c = [0.0f32; EMBEDDING_DIM];
    for v in cloud {
        for i in 0..EMBEDDING_DIM {
            c[i] += v[i];
        }
    }
    let n = cloud.len().max(1) as f32;
    for v in &mut c {
        *v /= n;
    }
    c
}

fn mean_distance(cloud: &[[f32; EMBEDDING_DIM]], centroid: &[f32; EMBEDDING_DIM]) -> f32 {
    if cloud.is_empty() {
        return 0.0;
    }
    cloud.iter().map(|v| vector_norm_diff(v, centroid)).sum::<f32>() / cloud.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn te_returns_zero_for_short_histories() {
        // With TE_MIN_SAMPLES = 128, short histories gracefully return 0.0
        let a = vec![[0.0; EMBEDDING_DIM]]; 
        let b = vec![[0.0; EMBEDDING_DIM]];
        assert_eq!(transfer_entropy(&a, &b), 0.0);
    }

    #[test]
    fn te_returns_zero_below_min_samples() {
        // 32 samples is still below TE_MIN_SAMPLES (128) — should return 0.0
        let a: Vec<[f32; EMBEDDING_DIM]> = (0..32).map(|_| [0.5; EMBEDDING_DIM]).collect();
        let b = a.clone();
        assert_eq!(transfer_entropy(&a, &b), 0.0);
    }

    #[test]
    fn te_returns_zero_for_identical_histories() {
        // Need ≥ TE_MIN_SAMPLES (128) samples to get past the guard
        let a: Vec<[f32; EMBEDDING_DIM]> = (0..200).map(|_| [0.5; EMBEDDING_DIM]).collect();
        let b = a.clone();
        // Identical, static histories should produce near-zero TE
        let te = transfer_entropy(&a, &b);
        // With zero deltas, te_sum should be 0
        assert!(te.abs() < 1.0, "static histories should have low TE, got {}", te);
    }

    #[test]
    fn detect_te_spikes_empty_input() {
        let spikes = detect_te_spikes(&[]);
        assert!(spikes.is_empty());
    }

    #[test]
    fn persistent_clusters_false_for_short_history() {
        let history = vec![vec![[0.0; EMBEDDING_DIM]]; 3];
        assert!(!detect_persistent_clusters(&history));
    }

    #[test]
    fn emergence_detection_none_for_empty_inputs() {
        let result = check_emergence(&[], &[]);
        assert_eq!(result, EmergenceDetection::None);
    }
}

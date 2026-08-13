//! Operational modes — the Director's repertoire.
//!
//! The Director operates in several distinct modes, switchable mid-performance.
//! Each mode has a characteristic feel profile, smoothing rate, and confidence.
//!
//! See: Director Design §9 "The Director's Repertoire"

use crate::protocol::FeelSpace;
use super::{feel_space, perception::PerceptionState};

/// The Director's operational mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectorMode {
    /// Follows the score's annotated dynamics and tempi closely.
    /// Moderate stubbornness, low risk.
    Conductor,

    /// High instrument autonomy, moderate coupling.
    /// Rhythm section locks, soloists are free. High risk/exploration.
    JazzBandleader,

    /// Human director takes primary control via a control surface.
    /// Oracle recedes to advisory mode.
    Painting,

    /// No score, no human input. Director generates its own narrative arc.
    /// Emergence is the primary creative engine.
    Generative,

    /// Maximum risk, maximum coupling. Pushed to the edge of chaos.
    /// For climactic moments.
    Storm,
}

impl Default for DirectorMode {
    fn default() -> Self {
        Self::JazzBandleader
    }
}

impl DirectorMode {
    /// Compute the target FeelSpace for this mode, given current perception.
    pub fn compute_feel(&self, perception: &PerceptionState) -> FeelSpace {
        let mut base = match self {
            Self::Conductor => feel_space::conductor_feel(),
            Self::JazzBandleader => feel_space::jazz_feel(),
            Self::Painting => FeelSpace::default(), // human controls — neutral base
            Self::Generative => FeelSpace {
                rho: 0.4,
                epsilon: 0.0,
                sigma: 0.0,
                tau: 0.55,
                gamma: 0.4,
                lambda: 0.5,
                phi: [0.0, 0.0],
            },
            Self::Storm => feel_space::storm_feel(),
        };

        // React to perception: if dispersion is dangerously high, increase coupling
        if perception.dispersion > 0.5 {
            base.gamma = (base.gamma + 0.2).min(1.0);
            base.lambda = (base.lambda - 0.2).max(0.0);
        }

        // If coherence is high, we can afford more exploration
        if perception.coherence > 0.8 {
            base.lambda = (base.lambda + 0.1).min(1.0);
        }

        base.clamp();
        base
    }

    /// Exponential smoothing alpha for this mode.
    /// Higher = more responsive, lower = smoother.
    pub fn smoothing_alpha(&self) -> f32 {
        match self {
            Self::Conductor => 0.15,   // deliberate changes
            Self::JazzBandleader => 0.25, // moderate responsiveness
            Self::Painting => 0.4,     // responsive to human input
            Self::Generative => 0.1,   // slow, organic evolution
            Self::Storm => 0.5,        // fast, volatile
        }
    }

    /// Director confidence for this mode — how strongly to apply tilt.
    pub fn confidence(&self) -> f32 {
        match self {
            Self::Conductor => 0.8,    // insist — follow the score
            Self::JazzBandleader => 0.5, // moderate — let instruments breathe
            Self::Painting => 0.3,     // whisper — human is in charge
            Self::Generative => 0.4,   // gentle guidance
            Self::Storm => 0.7,        // strong — commit to the storm
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::EMBEDDING_DIM;

    fn neutral_perception() -> PerceptionState {
        PerceptionState::default()
    }

    #[test]
    fn conductor_has_low_lambda() {
        let p = neutral_perception();
        let feel = DirectorMode::Conductor.compute_feel(&p);
        assert!(feel.lambda <= 0.2, "conductor mode should have low risk");
    }

    #[test]
    fn storm_has_high_gamma_and_lambda() {
        let p = neutral_perception();
        let feel = DirectorMode::Storm.compute_feel(&p);
        assert!(feel.gamma > 0.8);
        assert!(feel.lambda > 0.8);
    }

    #[test]
    fn jazz_has_moderate_values() {
        let p = neutral_perception();
        let feel = DirectorMode::JazzBandleader.compute_feel(&p);
        assert!(feel.gamma > 0.3 && feel.gamma < 0.8);
        assert!(feel.lambda > 0.2);
    }

    #[test]
    fn high_dispersion_increases_coupling() {
        let mut p = neutral_perception();
        let low_disp_feel = DirectorMode::JazzBandleader.compute_feel(&p);

        p.dispersion = 0.8;
        let high_disp_feel = DirectorMode::JazzBandleader.compute_feel(&p);

        assert!(high_disp_feel.gamma >= low_disp_feel.gamma,
            "High dispersion should increase coupling");
        assert!(high_disp_feel.lambda <= low_disp_feel.lambda,
            "High dispersion should decrease risk");
    }

    #[test]
    fn smoothing_alpha_is_in_valid_range() {
        for mode in [
            DirectorMode::Conductor,
            DirectorMode::JazzBandleader,
            DirectorMode::Painting,
            DirectorMode::Generative,
            DirectorMode::Storm,
        ] {
            let a = mode.smoothing_alpha();
            assert!(a > 0.0 && a <= 1.0, "alpha out of range for {:?}", mode);
        }
    }

    #[test]
    fn confidence_is_in_valid_range() {
        for mode in [
            DirectorMode::Conductor,
            DirectorMode::JazzBandleader,
            DirectorMode::Painting,
            DirectorMode::Generative,
            DirectorMode::Storm,
        ] {
            let c = mode.confidence();
            assert!(c >= 0.0 && c <= 1.0, "confidence out of range for {:?}", mode);
        }
    }

    #[test]
    fn default_mode_is_jazz() {
        assert_eq!(DirectorMode::default(), DirectorMode::JazzBandleader);
    }
}

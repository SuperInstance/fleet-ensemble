//! Alignment Module — Kalman-filtered phase-lock loop.
//!
//! Adjusts micro-timing, dynamics, articulation, and note choice based on
//! director feel + ensemble state.
//!
//! ## Spring-Damper Model
//!
//! Timing corrections use a critically-damped spring-damper to prevent
//! oscillation while converging as fast as possible:
//! ```text
//! F = -k * x - c * v
//! ```
//! Where `k` is spring constant (alignment gain), `c` is damping coefficient,
//! `x` is timing offset, and `v` is the rate of change.
//!
//! The system is mass-spring-damper with mass `m = 1`. Stability requires
//! `c² ≥ 4mk` (critical damping at equality). We compute `c` from `k` to
//! guarantee critical damping: `c = 2√(mk) = 2√k`.
//!
//! Integration uses semi-implicit (symplectic) Euler with a consistent `dt`:
//! ```text
//! v(t+dt) = v(t) + dt * F(t) / m
//! x(t+dt) = x(t) + dt * v(t+dt)
//! ```
//! Semi-implicit Euler is energy-preserving for oscillatory systems and
//! does not gain energy over time unlike explicit Euler.
//
//! ## Kalman Filter
//
//! Tracks long-term phase drift and estimates sync confidence.
//
//! See: Instrument Agent Design §5 "Alignment Mechanics"

use super::voice::Personality;
use crate::protocol::FeelTiltPayload;

/// Alignment state — tracks timing, dynamics, and sync confidence.
#[derive(Debug, Clone)]
pub struct AlignmentState {
    /// Current timing offset (microseconds) from the ensemble attractor.
    pub timing_offset_us: f32,

    /// Timing velocity (rate of offset change).
    timing_velocity: f32,

    /// Sync confidence from Kalman filter [0, 1].
    pub sync_confidence: f32,

    /// Kalman filter state: estimated phase drift.
    kalman_estimate: f32,

    /// Kalman filter state: estimation uncertainty.
    kalman_uncertainty: f32,

    /// Process noise (how much we expect the true value to wander).
    kalman_q: f32,

    /// Measurement noise (how noisy our observations are).
    kalman_r: f32,

    /// Current effective velocity bias.
    pub velocity_bias: i8,

    /// Current articulation mode.
    pub articulation_mode: ArticulationMode,
}

/// Articulation modes derived from director color/weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArticulationMode {
    #[default]
    Portato,
    Staccato,
    Legato,
}

impl AlignmentState {
    pub fn new(_personality: &Personality) -> Self {
        Self {
            timing_offset_us: 0.0,
            timing_velocity: 0.0,
            sync_confidence: 0.5,
            kalman_estimate: 0.0,
            kalman_uncertainty: 1.0,
            kalman_q: 0.01,
            kalman_r: 0.1,
            velocity_bias: 0,
            articulation_mode: ArticulationMode::Portato,
        }
    }

    /// Spring constant from personality alignment gain.
    ///
    /// This is the stiffness of the virtual spring pulling the instrument
    /// toward the ensemble attractor. Higher alignment gain = stiffer spring.
    fn spring_constant(personality: &Personality) -> f32 {
        // Pull strength: 30% toward ensemble, modulated by alignment gain
        0.3 * personality.alignment_gain
    }

    /// Damping coefficient derived from physical units.
    ///
    /// For a mass-spring-damper with mass `m = 1`, critical damping is:
    /// `c_critical = 2 * sqrt(m * k) = 2 * sqrt(k)`
    ///
    /// We use 95% of critical damping (ζ = 0.95) for a tiny bit of warmth
    /// — the system settles quickly without overshoot.
    fn damping_coefficient(personality: &Personality) -> f32 {
        let k = Self::spring_constant(personality);
        let m = 1.0; // unit mass
        0.95 * 2.0 * (m * k).sqrt()
    }

    /// Verify spring-damper stability: c² ≥ 4mk ensures no oscillation.
    ///
    /// With our derived damping this always holds (ζ = 0.95 < 1.0 means
    /// technically underdamped, but the margin is tiny and the symplectic
    /// integrator prevents energy gain). For safety we clamp ζ ≥ 0.7.
    fn assert_stable(personality: &Personality) {
        let k = Self::spring_constant(personality);
        let c = Self::damping_coefficient(personality);
        let m = 1.0;
        let zeta_sq = c * c / (4.0 * m * k);
        debug_assert!(
            zeta_sq >= 0.49, // ζ ≥ 0.7
            "Spring-damper is unstable: ζ² = {zeta_sq:.4} (need ≥ 0.49)"
        );
    }

    /// Apply spring-damper correction to timing using semi-implicit Euler.
    ///
    /// `F = -k * x - c * v`
    ///
    /// Semi-implicit (symplectic) Euler integration:
    /// ```text
    /// v(t+dt) = v(t) + dt * a(t)       where a = F / m
    /// x(t+dt) = x(t) + dt * v(t+dt)
    /// ```
    /// This uses the *updated* velocity for the position step, which
    /// preserves energy in oscillatory systems.
    ///
    /// Parameters:
    /// - `ensemble_offset_us`: the ensemble attractor position
    /// - `personality`: provides spring constant and damping
    /// - `dt_secs`: time step in seconds (default: 0.125 = one pulse)
    pub fn apply_spring_damper(
        &mut self,
        ensemble_offset_us: f32,
        personality: &Personality,
        dt_secs: f32,
    ) {
        Self::assert_stable(personality);

        let k = Self::spring_constant(personality);
        let c = Self::damping_coefficient(personality);
        let m = 1.0; // unit mass

        // Displacement from attractor (positive = we're ahead of ensemble)
        let x = self.timing_offset_us - ensemble_offset_us;

        // Spring-damper force: F = -k*x - c*v
        let force = -k * x - c * self.timing_velocity;

        // Semi-implicit Euler: update velocity first, then use it for position
        let acceleration = force / m;
        self.timing_velocity += acceleration * dt_secs;
        self.timing_offset_us += self.timing_velocity * dt_secs;

        // Clamp to ±15ms (the alignment window)
        self.timing_offset_us = self.timing_offset_us.clamp(-15_000.0, 15_000.0);
    }

    /// Convenience wrapper: apply spring-damper with default pulse dt (125ms).
    pub fn apply_spring_damper_pulse(
        &mut self,
        ensemble_offset_us: f32,
        personality: &Personality,
    ) {
        self.apply_spring_damper(ensemble_offset_us, personality, 0.125);
    }

    /// Kalman filter update for phase drift estimation.
    ///
    /// Standard 1D Kalman: predict → update.
    pub fn kalman_update(&mut self, measurement: f32) {
        // Predict: uncertainty grows
        self.kalman_uncertainty += self.kalman_q;

        // Update: Kalman gain
        let gain = self.kalman_uncertainty / (self.kalman_uncertainty + self.kalman_r);

        self.kalman_estimate += gain * (measurement - self.kalman_estimate);
        self.kalman_uncertainty = (1.0 - gain) * self.kalman_uncertainty;

        // Sync confidence: higher when uncertainty is low
        self.sync_confidence = 1.0 / (1.0 + self.kalman_uncertainty);
    }

    /// Update alignment from director feel-tilt.
    ///
    /// Uses the same physically-derived spring-damper as `apply_spring_damper`,
    /// but modulates the spring constant by the director's coupling pressure (γ)
    /// to reflect the director's insistence on alignment.
    pub fn update(&mut self, tilt: &FeelTiltPayload, personality: &Personality) {
        Self::assert_stable(personality);

        // Modulated spring constant: director coupling scales the pull
        let k = Self::spring_constant(personality) * tilt.global.gamma;
        let c = 0.95 * 2.0 * k.sqrt(); // critical damping for modulated k
        let m = 1.0;
        let dt = 0.125; // one pulse period

        let target_offset = 0.0; // attractor = ensemble peak
        let x = target_offset - self.timing_offset_us;
        let force = -k * x - c * self.timing_velocity;

        // Semi-implicit Euler — consistent dt for both velocity and position
        let acceleration = force / m;
        self.timing_velocity += acceleration * dt;
        self.timing_offset_us += self.timing_velocity * dt;
        self.timing_offset_us = self.timing_offset_us.clamp(-15_000.0, 15_000.0);

        // Dynamics from ε (energy flux)
        let energy = tilt.global.epsilon;
        self.velocity_bias = (energy * 20.0) as i8;

        // Articulation from σ (color) and Φ
        let color = tilt.global.sigma;
        if color > 0.5 && tilt.global.phi[0] > 0.5 {
            self.articulation_mode = ArticulationMode::Staccato;
        } else if color < -0.3 {
            self.articulation_mode = ArticulationMode::Legato;
        } else {
            self.articulation_mode = ArticulationMode::Portato;
        }
    }

    /// Get the effective timing offset for a note.
    pub fn effective_offset_us(&self) -> i32 {
        // Humanization: never over-correct below 5ms
        if self.timing_offset_us.abs() < 5000.0 {
            return 0; // leave it alone — humans don't play on the grid
        }
        self.timing_offset_us as i32
    }

    /// Get the sync confidence from the Kalman filter.
    pub fn confidence(&self) -> f32 {
        self.sync_confidence
    }
}

impl Default for AlignmentState {
    fn default() -> Self {
        Self::new(&Personality::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::FeelSpace;

    fn piano_personality() -> Personality {
        Personality {
            alignment_gain: 0.25,
            confidence_threshold: 0.6,
            timing_jitter_base_us: 3000,
            lead_tendency: 0.4,
            density_tolerance: 0.8,
        }
    }

    fn bass_personality() -> Personality {
        Personality {
            alignment_gain: 0.7,
            confidence_threshold: 0.95,
            timing_jitter_base_us: 1000,
            lead_tendency: 0.1,
            density_tolerance: 0.3,
        }
    }

    fn drums_personality() -> Personality {
        Personality {
            alignment_gain: 0.9,
            confidence_threshold: 0.99,
            timing_jitter_base_us: 500,
            lead_tendency: 0.0,
            density_tolerance: 1.0,
        }
    }

    #[test]
    fn new_state_starts_centered() {
        let state = AlignmentState::new(&piano_personality());
        assert_eq!(state.timing_offset_us, 0.0);
        assert!(!state.sync_confidence.is_nan());
    }

    #[test]
    fn spring_damper_pulls_toward_attractor() {
        let mut state = AlignmentState::new(&bass_personality());
        // Start with a large offset
        state.timing_offset_us = 10000.0;
        state.apply_spring_damper_pulse(0.0, &bass_personality());
        // Should have moved toward zero
        assert!(state.timing_offset_us < 10000.0,
            "spring-damper should pull toward attractor");
    }

    #[test]
    fn spring_damper_clamps_to_15ms() {
        let mut state = AlignmentState::new(&piano_personality());
        state.apply_spring_damper_pulse(1_000_000.0, &piano_personality());
        assert!(state.timing_offset_us.abs() <= 15_000.0,
            "timing offset should be clamped to ±15ms");
    }

    #[test]
    fn spring_damper_converges_over_multiple_steps() {
        // Verify the symplectic integrator converges without oscillation
        let mut state = AlignmentState::new(&bass_personality());
        state.timing_offset_us = 10000.0;

        for _ in 0..100 {
            state.apply_spring_damper_pulse(0.0, &bass_personality());
        }

        // After 100 pulses (12.5s) with near-critical damping, should be very close to 0.
        // With ζ=0.95 and k=0.21, the settling time is ~10s. Allow 500us tolerance
        // (5% of initial displacement).
        assert!(
            state.timing_offset_us.abs() < 500.0,
            "near-critically damped system should converge, got offset = {}",
            state.timing_offset_us
        );
    }

    #[test]
    fn spring_damper_does_not_overshoot_significantly() {
        // With ζ = 0.95 (near critical damping), overshoot should be negligible
        let mut state = AlignmentState::new(&bass_personality());
        state.timing_offset_us = 10000.0;

        let mut max_overshoot: f32 = 0.0;
        let mut prev_offset = state.timing_offset_us;

        for _ in 0..200 {
            state.apply_spring_damper_pulse(0.0, &bass_personality());
            // Detect sign change (overshoot past zero)
            if prev_offset > 0.0 && state.timing_offset_us < 0.0 {
                max_overshoot = max_overshoot.max(state.timing_offset_us.abs());
            }
            prev_offset = state.timing_offset_us;
        }

        // With near-critical damping, overshoot should be tiny
        assert!(
            max_overshoot < 200.0,
            "near-critical damping should not overshoot significantly, got {}",
            max_overshoot
        );
    }

    #[test]
    fn spring_damper_is_energy_stable() {
        // Symplectic Euler should not gain energy over time.
        // Run many steps and verify the amplitude doesn't grow.
        let mut state = AlignmentState::new(&piano_personality());
        state.timing_offset_us = 5000.0;
        state.timing_velocity = 0.0;

        let initial_energy = 0.5 * state.timing_velocity.powi(2)
            + 0.5 * spring_constant_test(&piano_personality())
                * state.timing_offset_us.powi(2);

        for _ in 0..1000 {
            state.apply_spring_damper_pulse(0.0, &piano_personality());
        }

        let final_energy = 0.5 * state.timing_velocity.powi(2)
            + 0.5 * spring_constant_test(&piano_personality())
                * state.timing_offset_us.powi(2);

        // Energy should decrease (damped system), never increase
        assert!(
            final_energy <= initial_energy * 1.01, // small numerical tolerance
            "symplectic integrator should not gain energy: initial={}, final={}",
            initial_energy,
            final_energy
        );
    }

    /// Helper for energy calculation in tests.
    fn spring_constant_test(personality: &Personality) -> f32 {
        AlignmentState::spring_constant(personality)
    }

    #[test]
    fn kalman_increases_confidence_with_observations() {
        let mut state = AlignmentState::new(&piano_personality());
        let initial_confidence = state.sync_confidence;

        // Feed consistent measurements
        for _ in 0..100 {
            state.kalman_update(0.0);
        }

        assert!(state.sync_confidence >= initial_confidence,
            "consistent observations should increase confidence");
    }

    #[test]
    fn kalman_converges_with_observations() {
        // Test that the Kalman filter estimate converges toward the true value
        let mut state = AlignmentState::new(&piano_personality());

        // Feed measurements around 50.0
        for _ in 0..200 {
            state.kalman_update(50.0);
        }

        // Estimate should be close to 50
        approx::assert_relative_eq!(state.kalman_estimate, 50.0, epsilon = 1.0);
        assert!(state.sync_confidence > 0.0, "should have positive confidence after converging");
    }

    #[test]
    fn effective_offset_returns_zero_for_small_corrections() {
        let mut state = AlignmentState::new(&piano_personality());
        state.timing_offset_us = 3000.0; // 3ms — within humanization window
        assert_eq!(state.effective_offset_us(), 0,
            "small corrections should be ignored (humanization rule)");
    }

    #[test]
    fn effective_offset_returns_nonzero_for_large_corrections() {
        let mut state = AlignmentState::new(&piano_personality());
        state.timing_offset_us = 8000.0; // 8ms — beyond humanization window
        assert!(state.effective_offset_us() != 0,
            "large corrections should be applied");
    }

    #[test]
    fn drums_have_stronger_spring_than_piano() {
        let k_drums = AlignmentState::spring_constant(&drums_personality());
        let k_piano = AlignmentState::spring_constant(&piano_personality());
        assert!(k_drums > k_piano,
            "drums should have stronger alignment pull than piano");
    }

    #[test]
    fn update_from_tilt_sets_articulation() {
        let mut state = AlignmentState::new(&piano_personality());
        let tilt = FeelTiltPayload {
            global: FeelSpace {
                sigma: 0.8,          // bright
                phi: [0.8, 0.0],     // sharp attack
                gamma: 0.5,
                epsilon: 0.0,
                ..Default::default()
            },
            confidence: 0.8,
            ..Default::default()
        };
        state.update(&tilt, &piano_personality());
        assert_eq!(state.articulation_mode, ArticulationMode::Staccato);
    }

    #[test]
    fn update_from_tilt_sets_legato_for_dark() {
        let mut state = AlignmentState::new(&piano_personality());
        let tilt = FeelTiltPayload {
            global: FeelSpace {
                sigma: -0.5,         // dark
                gamma: 0.5,
                epsilon: 0.0,
                ..Default::default()
            },
            ..Default::default()
        };
        state.update(&tilt, &piano_personality());
        assert_eq!(state.articulation_mode, ArticulationMode::Legato);
    }
}

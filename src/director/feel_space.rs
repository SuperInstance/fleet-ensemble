//! Feel Space — the 7-dimensional manifold the Director outputs.
//!
//! `F = (ρ, ε, σ, τ, γ, λ, Φ)`
//!
//! | Symbol | Name              | Range      | Meaning                       |
//! |--------|-------------------|------------|-------------------------------|
//! | ρ      | Pulse Density     | [0, 1]     | Micro-timing variance         |
//! | ε      | Energy Flux       | [-1, +1]   | Dynamic change rate           |
//! | σ      | Harmonic Tilt     | [-1, +1]   | Consonance/dissonance pressure|
//! | τ      | Temporal Asymmetry| [0.5, 0.8] | Swing ratio                   |
//! | γ      | Coupling Pressure | [0, 1]     | Imitation/alignment strength  |
//! | λ      | Risk Appetite     | [0, 1]     | Stochastic exploration        |
//! | Φ      | Articulation      | ℝ²         | Attack/release bias           |
//!
//! See: Director Design §2 "The Feel Space"

use crate::protocol::FeelSpace;

/// Extension trait for FeelSpace computations used by the Director.
pub trait FeelSpaceExt {
    /// Compute derived "Space" parameter: `ρ × λ⁻¹`.
    /// How much silence/air between events.
    fn space(&self) -> f32;

    /// Compute derived "Color" parameter: `σ × Φ`.
    /// Bright/dark, dense/sparse feel quality.
    fn color(&self) -> f32;

    /// Compute derived "Depth" parameter: `γ × dispersion`.
    /// Perceived closeness/distance of texture.
    fn depth(&self, dispersion: f32) -> f32;

    /// Compute the "Weight" for a specific instrument.
    /// In the stub, this is just the epsilon offset.
    fn weight(&self, _instrument: &str) -> f32;
}

impl FeelSpaceExt for FeelSpace {
    fn space(&self) -> f32 {
        // More density (ρ) with less risk (λ) = more air/space
        let lambda_inv = if self.lambda > 0.01 {
            1.0 / self.lambda
        } else {
            1.0 / 0.01
        };
        (self.rho * lambda_inv).clamp(0.0, 10.0)
    }

    fn color(&self) -> f32 {
        self.sigma * self.phi[0]
    }

    fn depth(&self, dispersion: f32) -> f32 {
        self.gamma * dispersion
    }

    fn weight(&self, _instrument: &str) -> f32 {
        // Stub: no per-instrument weighting yet
        self.epsilon
    }
}

/// Create a FeelSpace for "Conductor" mode — follows score closely.
pub fn conductor_feel() -> FeelSpace {
    FeelSpace {
        rho: 0.15,          // tight timing
        epsilon: 0.0,       // neutral energy
        sigma: 0.1,         // slight consonance pull
        tau: 0.5,           // straight
        gamma: 0.6,         // moderate coupling
        lambda: 0.1,        // low risk — respect the score
        phi: [0.0, 0.0],    // neutral articulation
    }
}

/// Create a FeelSpace for "Jazz Bandleader" mode — high autonomy, moderate coupling.
pub fn jazz_feel() -> FeelSpace {
    FeelSpace {
        rho: 0.35,          // loose timing
        epsilon: 0.1,       // gentle energy
        sigma: -0.1,        // slight dissonance tolerance
        tau: 0.62,          // moderate swing
        gamma: 0.5,         // moderate coupling — rhythm section locks, soloists free
        lambda: 0.4,        // high exploration
        phi: [0.2, -0.1],   // slight attack
    }
}

/// Create a FeelSpace for "Storm" mode — max risk, max coupling.
pub fn storm_feel() -> FeelSpace {
    FeelSpace {
        rho: 0.8,           // dense polyrhythm
        epsilon: 0.5,       // crescendo surge
        sigma: -0.5,        // dissonance
        tau: 0.55,          // slight swing
        gamma: 0.9,         // maximum coupling
        lambda: 0.9,        // maximum risk
        phi: [0.8, 0.3],    // sharp attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conductor_feel_is_conservative() {
        let fs = conductor_feel();
        assert!(fs.lambda < 0.2);
        assert!(fs.rho < 0.3);
        assert_eq!(fs.tau, 0.5); // straight
    }

    #[test]
    fn jazz_feel_has_swing() {
        let fs = jazz_feel();
        assert!(fs.tau > 0.55);
        assert!(fs.lambda > 0.3);
    }

    #[test]
    fn storm_feel_is_extreme() {
        let fs = storm_feel();
        assert!(fs.gamma > 0.8);
        assert!(fs.lambda > 0.8);
    }

    #[test]
    fn space_inversely_proportional_to_lambda() {
        let mut fs = FeelSpace::default();
        fs.rho = 0.5;
        fs.lambda = 0.5;
        let space_05 = fs.space();

        fs.lambda = 0.1;
        let space_01 = fs.space();

        // Lower λ should yield higher space (more air when less exploration)
        assert!(space_01 > space_05);
    }

    #[test]
    fn color_combines_sigma_and_phi() {
        let mut fs = FeelSpace::default();
        fs.sigma = 0.5;
        fs.phi = [0.4, 0.0];
        approx::assert_relative_eq!(fs.color(), 0.2, epsilon = 1e-6);
    }

    #[test]
    fn depth_combines_gamma_and_dispersion() {
        let mut fs = FeelSpace::default();
        fs.gamma = 0.7;
        let d = fs.depth(0.3);
        approx::assert_relative_eq!(d, 0.21, epsilon = 1e-6);
    }
}

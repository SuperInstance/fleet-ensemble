use serde::{Deserialize, Serialize};

/// A ternary vector with values in {-1, 0, +1}.
///
/// This represents discrete directional intent: -1 = decrease, 0 = hold, +1 = increase.
/// Ternary vectors are the discrete analog of continuous demand vectors, useful for
/// symbolic agent coordination and MIDI generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TernaryVector {
    /// The ternary values (-1, 0, or +1).
    pub values: Vec<i32>,
}

impl TernaryVector {
    /// Create a new ternary vector, validating all values are in {-1, 0, 1}.
    pub fn new(values: Vec<i32>) -> Self {
        assert!(values.iter().all(|&v| v == -1 || v == 0 || v == 1),
            "ternary values must be -1, 0, or +1");
        Self { values }
    }

    /// Create without validation (for internal use).
    pub fn new_unchecked(values: Vec<i32>) -> Self {
        Self { values }
    }

    /// Length of the vector.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Density: fraction of non-zero entries (how active the agent is).
    pub fn density(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        let nonzero = self.values.iter().filter(|&&v| v != 0).count();
        nonzero as f64 / self.values.len() as f64
    }

    /// Balance: (count of +1 minus count of -1) / total length.
    ///
    /// Positive = bullish, negative = bearish, zero = balanced.
    pub fn balance(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        let up = self.values.iter().filter(|&&v| v == 1).count();
        let down = self.values.iter().filter(|&&v| v == -1).count();
        (up as f64 - down as f64) / self.values.len() as f64
    }

    /// Convert to a sequence of MIDI note numbers starting from `base_pitch`.
    ///
    /// +1 → up 4 semitones (major third), -1 → down 4, 0 → repeat.
    /// This maps ternary intent to musical intervals.
    pub fn to_notes(&self, base_pitch: u8) -> Vec<u8> {
        let mut notes = vec![base_pitch];
        let mut current = base_pitch as i32;
        for &v in &self.values {
            current += v * 4; // ±4 semitones per step
            notes.push(current.clamp(0, 127) as u8);
        }
        notes
    }

    /// Convert to a continuous demand vector (scaling -1/0/+1 to f64).
    pub fn to_demand(&self) -> Vec<f64> {
        self.values.iter().map(|&v| v as f64).collect()
    }
}

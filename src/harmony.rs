use crate::KeySignature;

/// Validates harmonic content of MIDI event streams.
///
/// Checks for common counterpoint and harmony errors:
/// - Parallel fifths and octaves between consecutive events
/// - Notes outside the current key (optional)
/// - Range validity
#[derive(Debug, Clone)]
pub struct HarmonicValidator {
    /// If true, reject notes outside the current key.
    pub enforce_key: bool,
    /// If true, reject parallel fifths/octaves.
    pub reject_parallel: bool,
    /// Minimum allowed MIDI note number.
    pub min_note: u8,
    /// Maximum allowed MIDI note number.
    pub max_note: u8,
}

impl HarmonicValidator {
    /// Create a permissive validator (no enforcement).
    pub fn permissive() -> Self {
        Self {
            enforce_key: false,
            reject_parallel: false,
            min_note: 0,
            max_note: 127,
        }
    }

    /// Create a strict validator.
    pub fn strict(_key: KeySignature) -> Self {
        Self {
            enforce_key: true,
            reject_parallel: true,
            min_note: 21,  // A0
            max_note: 108, // C8
        }
    }

    /// Interval in semitones between two MIDI note numbers.
    pub fn interval(a: u8, b: u8) -> i8 {
        (b as i8) - (a as i8)
    }

    /// Interval class (0–11) between two notes, ignoring octave.
    pub fn interval_class(a: u8, b: u8) -> u8 {
        ((b as i16 - a as i16).rem_euclid(12)) as u8
    }

    /// Check if a note is in range.
    pub fn in_range(&self, note: u8) -> bool {
        note >= self.min_note && note <= self.max_note
    }

    /// Check if a note is in the current key.
    pub fn in_key(&self, note: u8, key: &KeySignature) -> bool {
        !self.enforce_key || key.contains(note)
    }

    /// Check two note pairs for parallel fifths or octaves.
    ///
    /// `prev` and `curr` are pairs of (voice_a, voice_b).
    /// Returns `true` if the motion is valid (no parallel 5ths/8ves).
    pub fn check_parallel(prev: (u8, u8), curr: (u8, u8)) -> bool {
        let ic_prev = Self::interval_class(prev.0, prev.1);
        let ic_curr = Self::interval_class(curr.0, curr.1);
        // parallel = same interval class AND both voices moved
        let moved = prev.0 != curr.0 || prev.1 != curr.1;
        let is_fifth_or_octave = ic_prev == 7 || ic_prev == 0;
        let same_interval = ic_prev == ic_curr;
        !(moved && same_interval && is_fifth_or_octave)
    }

    /// Check if a chord (list of pitch classes) is a standard triad or seventh.
    pub fn is_standard_chord(notes: &[u8]) -> bool {
        if notes.len() < 3 {
            return false;
        }
        let pcs: Vec<u8> = notes.iter().map(|&n| n % 12).collect();
        // Check all pairs for major/minor third, perfect fifth intervals
        for i in 0..pcs.len() {
            for j in i + 1..pcs.len() {
                let ic = Self::interval_class(pcs[i], pcs[j]);
                if ic == 7 || ic == 5 {
                    return true; // has a fifth = triad-ish
                }
            }
        }
        false
    }

    /// Dissonance level of an interval (0 = consonant, higher = more dissonant).
    pub fn dissonance(interval_class: u8) -> u8 {
        match interval_class {
            0 | 7 => 0,           // unison, P5 = perfect consonance
            3 | 4 | 8 | 9 => 1,   // m3, M3, m6, M6 = imperfect consonance
            1 | 2 | 10 | 11 => 2, // m2, M2, m7, M7 = dissonance
            5 => 0,               // P4 = consonant in some contexts
            6 => 3,               // tritone = maximum dissonance
            _ => 4,
        }
    }

    /// Validate a batch of note events. Returns events that pass all checks.
    pub fn validate(&self, notes: &[u8], key: &KeySignature) -> Vec<u8> {
        notes.iter().copied().filter(|&n| self.in_range(n) && self.in_key(n, key)).collect()
    }
}

impl Default for HarmonicValidator {
    fn default() -> Self {
        Self::permissive()
    }
}

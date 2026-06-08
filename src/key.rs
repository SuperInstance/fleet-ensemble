/// Musical mode (Major or Minor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    Major,
    Minor,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Major => write!(f, "Major"),
            Self::Minor => write!(f, "Minor"),
        }
    }
}

/// Key signature with root note and mode.
///
/// `root` uses MIDI note-number pitch classes: 0 = C, 1 = C#, …, 11 = B.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeySignature {
    /// Root pitch class (0=C … 11=B).
    pub root: u8,
    /// Major or Minor mode.
    pub mode: Mode,
}

/// Major scale intervals in semitones from root.
const MAJOR_INTERVALS: &[u8] = &[0, 2, 4, 5, 7, 9, 11];
/// Natural minor scale intervals in semitones from root.
const MINOR_INTERVALS: &[u8] = &[0, 2, 3, 5, 7, 8, 10];

impl KeySignature {
    /// Create a new key signature.
    pub fn new(root: u8, mode: Mode) -> Self {
        Self { root: root % 12, mode }
    }

    /// C Major.
    pub fn c_major() -> Self { Self::new(0, Mode::Major) }
    /// A Minor.
    pub fn a_minor() -> Self { Self::new(9, Mode::Minor) }

    /// Return the scale pitch classes (7 notes) for this key.
    pub fn scale(&self) -> Vec<u8> {
        let intervals = match self.mode {
            Mode::Major => MAJOR_INTERVALS,
            Mode::Minor => MINOR_INTERVALS,
        };
        intervals.iter().map(|&i| (self.root + i) % 12).collect()
    }

    /// Check if a pitch class belongs to this key.
    pub fn contains(&self, note: u8) -> bool {
        let pc = note % 12;
        self.scale().contains(&pc)
    }

    /// Transpose a pitch class into this key (find nearest scale tone).
    ///
    /// Returns the closest scale degree above or at `note`.
    pub fn nearest_scale_tone(&self, note: u8) -> u8 {
        let pc = note % 12;
        let scale = self.scale();
        // find the smallest distance
        let mut best = scale[0];
        let mut best_dist = 12u8;
        for &s in &scale {
            let dist = (s as i16 - pc as i16).rem_euclid(12) as u8;
            if dist < best_dist {
                best_dist = dist;
                best = s;
            }
        }
        best
    }

    /// Degree index (0–6) of a pitch class in this key, or `None` if chromatic.
    pub fn degree_of(&self, note: u8) -> Option<usize> {
        let pc = note % 12;
        self.scale().iter().position(|&n| n == pc)
    }

    /// Roman-numeral degree name for a pitch class.
    pub fn roman(&self, note: u8) -> Option<&'static str> {
        self.degree_of(note).map(|d| match d {
            0 => "I", 1 => "II", 2 => "III", 3 => "IV",
            4 => "V", 5 => "VI", 6 => "VII", _ => "?",
        })
    }

    /// Modulate to a new root, keeping or changing mode.
    pub fn modulate(&mut self, new_root: u8, new_mode: Option<Mode>) {
        self.root = new_root % 12;
        if let Some(m) = new_mode {
            self.mode = m;
        }
    }

    /// Pitch class name (C, C#, D, …, B).
    pub fn note_name(note: u8) -> &'static str {
        match note % 12 {
            0 => "C", 1 => "C#", 2 => "D", 3 => "D#", 4 => "E", 5 => "F",
            6 => "F#", 7 => "G", 8 => "G#", 9 => "A", 10 => "A#", 11 => "B",
            _ => "?",
        }
    }

    /// Human-readable key name, e.g. "C Major".
    pub fn name(&self) -> String {
        format!("{} {}", Self::note_name(self.root), self.mode)
    }
}

impl Default for KeySignature {
    fn default() -> Self {
        Self::c_major()
    }
}

/// Tracks tempo, tick position, swing, and derived timing info.
///
/// `ticks_per_beat` defines the resolution (typically 480 for MIDI).
/// `swing` ranges from 0.0 (straight) through ~0.33 (light) to ~0.66 (heavy).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TempoTracker {
    /// Beats per minute.
    pub bpm: f64,
    /// Ticks per quarter-note beat (PPQ resolution).
    pub ticks_per_beat: u32,
    /// Current tick counter (monotonically increasing).
    pub tick: u64,
    /// Swing amount: 0.0 = straight, 0.33 = light, 0.66 = heavy.
    pub swing: f64,
}

impl TempoTracker {
    /// Create a new tempo tracker at the given BPM and PPQ resolution.
    pub fn new(bpm: f64, ticks_per_beat: u32) -> Self {
        Self { bpm, ticks_per_beat, tick: 0, swing: 0.0 }
    }

    /// Advance by one tick. Returns the new tick value.
    pub fn advance(&mut self) -> u64 {
        self.tick += 1;
        self.tick
    }

    /// Current beat (zero-based) within the tempo stream.
    pub fn beat(&self) -> u64 {
        self.tick / self.ticks_per_beat as u64
    }

    /// Current tick within the current beat (0 .. ticks_per_beat).
    pub fn beat_phase(&self) -> u32 {
        (self.tick % self.ticks_per_beat as u64) as u32
    }

    /// Current bar (assuming 4/4 time), zero-based.
    pub fn bar(&self) -> u64 {
        self.beat() / 4
    }

    /// Beat within the current bar (0–3 for 4/4).
    pub fn beat_in_bar(&self) -> u64 {
        self.beat() % 4
    }

    /// Whether the current tick falls on a strong beat (beat 0 or 2 in 4/4).
    pub fn is_strong_beat(&self) -> bool {
        self.beat_phase() == 0 && (self.beat_in_bar() == 0 || self.beat_in_bar() == 2)
    }

    /// Whether the current tick falls on a downbeat (beat 0 of a bar).
    pub fn is_downbeat(&self) -> bool {
        self.beat_phase() == 0 && self.beat_in_bar() == 0
    }

    /// Whether the current tick falls on a weak beat.
    pub fn is_weak_beat(&self) -> bool {
        self.beat_phase() == 0 && !self.is_strong_beat()
    }

    /// Apply swing to a beat-phase value.
    ///
    /// Returns the swing-adjusted phase. Off-beats are delayed by
    /// `swing * ticks_per_beat / 2`.
    pub fn swing_offset(&self) -> u32 {
        let beat = self.beat() as u32;
        // swing only applies to off-beats (odd-numbered eighth notes)
        if beat % 2 == 1 {
            (self.swing * self.ticks_per_beat as f64 * 0.5) as u32
        } else {
            0
        }
    }

    /// Duration of one tick in seconds.
    pub fn tick_duration_secs(&self) -> f64 {
        60.0 / (self.bpm * self.ticks_per_beat as f64)
    }

    /// Reset to tick 0.
    pub fn reset(&mut self) {
        self.tick = 0;
    }
}

impl Default for TempoTracker {
    fn default() -> Self {
        Self::new(120.0, 480)
    }
}

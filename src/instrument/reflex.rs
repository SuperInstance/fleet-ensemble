//! Reflex Engine — <10ms hard-coded musical responses.
//!
//! Fast algorithmic responses that bypass all neural inference.
//! These are the "muscle memory" of the instrument — kicks that fire
//! before conscious perception can intervene.
//!
//! Examples:
//! - If kick fires, snare follows within 2ms
//! - If bass plays root, piano drops root from chord
//! - If ensemble density exceeds threshold, switch to comping
//!
//! See: Instrument Agent Design §6 "Concrete Instrument Designs"

use super::voice::{ReflexResponse, VoiceClass};

/// A musical event that might trigger a reflex.
#[derive(Debug, Clone, PartialEq)]
pub struct ReflexEvent {
    /// Which instrument sent the event.
    pub source_id: u16,
    /// Source voice class.
    pub source_voice: VoiceClass,
    /// MIDI pitch.
    pub pitch: u8,
    /// MIDI velocity (0-127).
    pub velocity: u8,
    /// Timestamp in microseconds.
    pub timestamp_us: u64,
    /// True if this is a chord (multiple simultaneous notes).
    pub is_chord: bool,
    /// True if the event falls on a beat boundary.
    pub on_beat: bool,
}

/// The reflex engine — evaluates hard-coded musical reflexes.
pub struct ReflexEngine {
    /// Current ensemble density (0.0 = silent, 1.0 = everyone playing loud).
    ensemble_density: f32,
    /// Pending velocity boost for next attack.
    next_attack_boost: i8,
    /// Whether to thin upcoming intents.
    thin_factor: f32,
}

impl Default for ReflexEngine {
    fn default() -> Self {
        Self {
            ensemble_density: 0.0,
            next_attack_boost: 0,
            thin_factor: 1.0,
        }
    }
}

impl ReflexEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the ensemble density estimate.
    pub fn set_density(&mut self, density: f32) {
        self.ensemble_density = density.clamp(0.0, 1.0);
    }

    /// Evaluate a reflex event from a peer.
    ///
    /// Returns a list of reflex responses to fire, if any.
    pub fn evaluate(&mut self, event: &ReflexEvent) -> Vec<ReflexResponse> {
        let mut responses = Vec::new();

        // Piano reflexes
        if matches!(event.source_voice, VoiceClass::Bass) {
            if event.pitch >= 28 && event.pitch <= 60 {
                responses.push(ReflexResponse::DropRootFromChord);
            }
        }

        if matches!(event.source_voice, VoiceClass::Drums) {
            // Crash pitches: 49, 57
            if event.pitch == 49 || event.pitch == 57 {
                self.next_attack_boost = self.next_attack_boost.saturating_add(15);
            }
        }

        if matches!(event.source_voice, VoiceClass::Piano) && event.is_chord && event.on_beat {
            responses.push(ReflexResponse::BrightenGhostNotes);
        }

        // Bass reflexes
        if matches!(event.source_voice, VoiceClass::Drums) {
            // Kick pitches: 35, 36
            if event.pitch == 35 || event.pitch == 36 {
                responses.push(ReflexResponse::AlignToKick);
            }
        }

        // Drum reflexes
        if matches!(event.source_voice, VoiceClass::Bass) {
            if event.pitch >= 28 && event.pitch <= 60 {
                responses.push(ReflexResponse::KickLockToBass);
            }
        }

        // Ensemble-level reflexes
        if self.ensemble_density > 0.8 {
            self.thin_factor = 0.5;
        } else {
            self.thin_factor = 1.0;
        }

        responses
    }

    /// Get the current attack boost (from crash-induced brightening).
    pub fn attack_boost(&self) -> i8 {
        self.next_attack_boost
    }

    /// Consume the attack boost (reset after applying).
    pub fn consume_attack_boost(&mut self) {
        self.next_attack_boost = 0;
    }

    /// Get the current note thinning factor.
    pub fn thin_factor(&self) -> f32 {
        self.thin_factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bass_event(pitch: u8) -> ReflexEvent {
        ReflexEvent {
            source_id: 2,
            source_voice: VoiceClass::Bass,
            pitch,
            velocity: 100,
            timestamp_us: 0,
            is_chord: false,
            on_beat: true,
        }
    }

    fn kick_event() -> ReflexEvent {
        ReflexEvent {
            source_id: 3,
            source_voice: VoiceClass::Drums,
            pitch: 36,
            velocity: 120,
            timestamp_us: 0,
            is_chord: false,
            on_beat: true,
        }
    }

    fn crash_event() -> ReflexEvent {
        ReflexEvent {
            source_id: 3,
            source_voice: VoiceClass::Drums,
            pitch: 49,
            velocity: 110,
            timestamp_us: 0,
            is_chord: false,
            on_beat: true,
        }
    }

    #[test]
    fn bass_event_triggers_drop_root() {
        let mut engine = ReflexEngine::new();
        let responses = engine.evaluate(&bass_event(40));
        assert!(responses.contains(&ReflexResponse::DropRootFromChord));
    }

    #[test]
    fn kick_event_triggers_align_to_kick() {
        let mut engine = ReflexEngine::new();
        let responses = engine.evaluate(&kick_event());
        assert!(responses.contains(&ReflexResponse::AlignToKick));
    }

    #[test]
    fn bass_event_triggers_kick_lock_for_drums() {
        let mut engine = ReflexEngine::new();
        let responses = engine.evaluate(&bass_event(40));
        // The KickLockToBass reflex fires when drums hear bass
        assert!(responses.contains(&ReflexResponse::KickLockToBass));
    }

    #[test]
    fn crash_event_sets_attack_boost() {
        let mut engine = ReflexEngine::new();
        engine.evaluate(&crash_event());
        assert!(engine.attack_boost() > 0, "crash should boost attack");
    }

    #[test]
    fn consume_attack_boost_resets() {
        let mut engine = ReflexEngine::new();
        engine.evaluate(&crash_event());
        engine.consume_attack_boost();
        assert_eq!(engine.attack_boost(), 0);
    }

    #[test]
    fn high_density_thins_intents() {
        let mut engine = ReflexEngine::new();
        engine.set_density(0.9);
        // Evaluating any event triggers density check
        engine.evaluate(&bass_event(40));
        assert!(engine.thin_factor() < 1.0, "high density should thin");
    }

    #[test]
    fn low_density_does_not_thin() {
        let mut engine = ReflexEngine::new();
        engine.set_density(0.3);
        engine.evaluate(&bass_event(40));
        assert_eq!(engine.thin_factor(), 1.0);
    }

    #[test]
    fn density_is_clamped() {
        let mut engine = ReflexEngine::new();
        engine.set_density(5.0);
        engine.evaluate(&bass_event(40));
        engine.set_density(-1.0);
        // Should not panic
    }
}

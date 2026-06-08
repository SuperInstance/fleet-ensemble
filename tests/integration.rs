#[cfg(test)]
mod tests {
    use fleet_ensemble::*;

    // --- Role tests ---

    #[test]
    fn role_ordering() {
        assert!(AgentRole::Conductor < AgentRole::Guardian);
        assert!(AgentRole::Guardian < AgentRole::Critic);
        assert!(AgentRole::Builder < AgentRole::Explorer);
        assert!(AgentRole::Narrator > AgentRole::Integrator);
    }

    #[test]
    fn role_priority_values() {
        assert_eq!(AgentRole::Conductor.priority(), 0);
        assert_eq!(AgentRole::Narrator.priority(), 7);
    }

    #[test]
    fn role_display() {
        assert_eq!(format!("{}", AgentRole::Builder), "Builder");
    }

    #[test]
    fn role_name() {
        assert_eq!(AgentRole::Explorer.name(), "Explorer");
    }

    // --- Agent tests ---

    #[test]
    fn agent_creation() {
        let a = EnsembleAgent::new("test".into(), AgentRole::Builder, 1, InstrumentPatch::piano());
        assert_eq!(a.id, "test");
        assert_eq!(a.channel, 1);
        assert!(!a.muted);
        assert!(!a.solo);
    }

    #[test]
    fn agent_mute_solo_builders() {
        let a = EnsembleAgent::new("x".into(), AgentRole::Critic, 2, InstrumentPatch::strings())
            .muted(true)
            .solo(true);
        assert!(a.muted);
        assert!(a.solo);
    }

    #[test]
    fn agent_display() {
        let a = EnsembleAgent::new("p1".into(), AgentRole::Conductor, 0, InstrumentPatch::piano());
        let s = format!("{}", a);
        assert!(s.contains("Conductor"));
        assert!(s.contains("p1"));
    }

    #[test]
    fn instrument_patches() {
        let p = InstrumentPatch::piano();
        assert_eq!(p.program, 0);
        let b = InstrumentPatch::bass();
        assert_eq!(b.program, 33);
        let d = InstrumentPatch::drums();
        assert_eq!(d.bank, 120);
    }

    // --- Tempo tests ---

    #[test]
    fn tempo_advance() {
        let mut t = TempoTracker::new(120.0, 480);
        assert_eq!(t.tick, 0);
        t.advance();
        assert_eq!(t.tick, 1);
    }

    #[test]
    fn tempo_beat_and_bar() {
        let mut t = TempoTracker::new(120.0, 480);
        // tick 0 = beat 0, bar 0
        assert_eq!(t.beat(), 0);
        assert_eq!(t.bar(), 0);
        assert_eq!(t.beat_in_bar(), 0);
        assert!(t.is_downbeat());
        assert!(t.is_strong_beat());

        // advance to tick 480 = beat 1, bar 0
        for _ in 0..480 { t.advance(); }
        assert_eq!(t.beat(), 1);
        assert_eq!(t.beat_in_bar(), 1);
        assert!(!t.is_strong_beat());
        assert!(t.is_weak_beat());

        // advance to tick 960 = beat 2
        for _ in 0..480 { t.advance(); }
        assert_eq!(t.beat(), 2);
        assert!(t.is_strong_beat());
    }

    #[test]
    fn tempo_beat_phase() {
        let mut t = TempoTracker::new(120.0, 480);
        for _ in 0..240 { t.advance(); }
        assert_eq!(t.beat_phase(), 240);
    }

    #[test]
    fn tempo_swing() {
        let mut t = TempoTracker::new(120.0, 480);
        t.swing = 0.66;
        // beat 0 (even) → no swing offset
        assert_eq!(t.swing_offset(), 0);
        // advance to beat 1 (odd)
        for _ in 0..480 { t.advance(); }
        let offset = t.swing_offset();
        assert!(offset > 0);
    }

    #[test]
    fn tempo_tick_duration() {
        let t = TempoTracker::new(120.0, 480);
        let dur = t.tick_duration_secs();
        assert!(dur > 0.0);
        // 120 BPM, 480 PPQ → 57600 ticks/sec → ~17.36 µs per tick
        assert!(dur < 0.002);
    }

    #[test]
    fn tempo_default() {
        let t = TempoTracker::default();
        assert_eq!(t.bpm, 120.0);
        assert_eq!(t.ticks_per_beat, 480);
    }

    #[test]
    fn tempo_reset() {
        let mut t = TempoTracker::new(100.0, 240);
        for _ in 0..1000 { t.advance(); }
        t.reset();
        assert_eq!(t.tick, 0);
    }

    // --- Key tests ---

    #[test]
    fn key_c_major_scale() {
        let k = KeySignature::c_major();
        assert_eq!(k.scale(), vec![0, 2, 4, 5, 7, 9, 11]); // C D E F G A B
    }

    #[test]
    fn key_a_minor_scale() {
        let k = KeySignature::a_minor();
        assert_eq!(k.scale(), vec![9, 11, 0, 2, 4, 5, 7]); // A B C D E F G
    }

    #[test]
    fn key_contains() {
        let k = KeySignature::c_major();
        assert!(k.contains(0));  // C
        assert!(k.contains(7));  // G
        assert!(!k.contains(1)); // C#
        assert!(!k.contains(6)); // F#
    }

    #[test]
    fn key_nearest_scale_tone() {
        let k = KeySignature::c_major();
        // C#(1) is equidistant from C(0) and D(2) — function returns closest
        let nearest = k.nearest_scale_tone(1);
        assert!(nearest == 0 || nearest == 2, "expected 0 or 2, got {nearest}");
        assert_eq!(k.nearest_scale_tone(6), 7); // F# → G
        assert_eq!(k.nearest_scale_tone(4), 4); // E → E
        assert_eq!(k.nearest_scale_tone(7), 7); // G → G
    }

    #[test]
    fn key_degree_of() {
        let k = KeySignature::c_major();
        assert_eq!(k.degree_of(0), Some(0)); // C = I
        assert_eq!(k.degree_of(7), Some(4)); // G = V
        assert_eq!(k.degree_of(1), None);    // C# not in scale
    }

    #[test]
    fn key_roman() {
        let k = KeySignature::c_major();
        assert_eq!(k.roman(0), Some("I"));
        assert_eq!(k.roman(7), Some("V"));
    }

    #[test]
    fn key_modulate() {
        let mut k = KeySignature::c_major();
        k.modulate(7, Some(Mode::Major)); // G Major
        assert_eq!(k.root, 7);
        assert_eq!(k.mode, Mode::Major);
    }

    #[test]
    fn key_name() {
        let k = KeySignature::new(7, Mode::Minor);
        assert_eq!(k.name(), "G Minor");
    }

    #[test]
    fn key_note_name() {
        assert_eq!(KeySignature::note_name(0), "C");
        assert_eq!(KeySignature::note_name(9), "A");
    }

    // --- Harmony tests ---

    #[test]
    fn harmony_interval() {
        assert_eq!(HarmonicValidator::interval(60, 67), 7);  // P5
        assert_eq!(HarmonicValidator::interval(60, 72), 12); // P8
    }

    #[test]
    fn harmony_interval_class() {
        assert_eq!(HarmonicValidator::interval_class(0, 7), 7);
        assert_eq!(HarmonicValidator::interval_class(60, 67), 7);
    }

    #[test]
    fn harmony_parallel_fifths() {
        // C-G → D-A (both P5, both moved) = parallel
        assert!(!HarmonicValidator::check_parallel((60, 67), (62, 69)));
        // C-G → C-G (same, no movement) = ok
        assert!(HarmonicValidator::check_parallel((60, 67), (60, 67)));
        // C-G → D-F# (P5 → M3) = ok
        assert!(HarmonicValidator::check_parallel((60, 67), (62, 66)));
    }

    #[test]
    fn harmony_parallel_octaves() {
        assert!(!HarmonicValidator::check_parallel((60, 72), (62, 74))); // P8 → P8
    }

    #[test]
    fn harmony_dissonance() {
        assert_eq!(HarmonicValidator::dissonance(0), 0);  // unison
        assert_eq!(HarmonicValidator::dissonance(7), 0);  // P5
        assert_eq!(HarmonicValidator::dissonance(6), 3);  // tritone
        assert_eq!(HarmonicValidator::dissonance(1), 2);  // m2
    }

    #[test]
    fn harmony_validate_permissive() {
        let v = HarmonicValidator::permissive();
        let k = KeySignature::c_major();
        let result = v.validate(&[60, 61, 62, 127], &k);
        assert_eq!(result, vec![60, 61, 62, 127]);
    }

    #[test]
    fn harmony_validate_strict() {
        let v = HarmonicValidator::strict(KeySignature::c_major());
        let k = KeySignature::c_major();
        // 61 = C#, not in C major
        let result = v.validate(&[60, 61, 62], &k);
        assert_eq!(result, vec![60, 62]);
    }

    #[test]
    fn harmony_range() {
        let v = HarmonicValidator::strict(KeySignature::c_major());
        assert!(v.in_range(60));
        assert!(!v.in_range(10));  // below A0=21
        assert!(!v.in_range(120)); // above C8=108
    }

    #[test]
    fn harmony_standard_chord() {
        assert!(HarmonicValidator::is_standard_chord(&[0, 4, 7])); // C major triad
        assert!(HarmonicValidator::is_standard_chord(&[0, 3, 7])); // C minor triad
        assert!(!HarmonicValidator::is_standard_chord(&[0, 1]));   // too few notes
    }

    // --- Stream tests ---

    #[test]
    fn stream_push_and_finalize() {
        let mut s = EventStream::new();
        s.push(MidiEvent::note_on(10, 0, 60, 100));
        s.push(MidiEvent::note_on(5, 0, 64, 100));
        s.finalize();
        assert_eq!(s.events()[0].tick, 5);
        assert_eq!(s.events()[1].tick, 10);
    }

    #[test]
    fn stream_duration() {
        let mut s = EventStream::new();
        assert_eq!(s.duration_ticks(), 0);
        s.push(MidiEvent::note_on(100, 0, 60, 100));
        s.push(MidiEvent::note_on(200, 0, 64, 100));
        assert_eq!(s.duration_ticks(), 200);
    }

    #[test]
    fn midi_event_note_off() {
        let e = MidiEvent::note_off(10, 1, 60, 0);
        assert!(e.is_note_off());
        assert!(!e.is_note_on());
    }

    #[test]
    fn midi_event_cc() {
        let e = MidiEvent::cc(0, 0, 64, 127); // sustain pedal
        assert_eq!(e.tick, 0);
        assert_eq!(e.channel, 0);
    }

    #[test]
    fn midi_event_display() {
        let e = MidiEvent::note_on(0, 0, 60, 100);
        let s = format!("{}", e);
        assert!(s.contains("NOTE_ON"));
    }

    // --- Resolver tests ---

    #[test]
    fn resolver_basic() {
        let r = ConflictResolver::new();
        let proposals = vec![
            Proposal { role: AgentRole::Explorer, agent_id: "e1".into(), event: MidiEvent::note_on(0, 0, 60, 100) },
            Proposal { role: AgentRole::Conductor, agent_id: "c1".into(), event: MidiEvent::note_on(0, 0, 64, 100) },
        ];
        let events = r.resolve(proposals);
        // Conductor's event should come first
        assert_eq!(events[0].pitch(), 64);
        assert_eq!(events[1].pitch(), 60);
    }

    #[test]
    fn resolver_winner_takes_channel() {
        let r = ConflictResolver::winner_takes_all();
        let proposals = vec![
            Proposal { role: AgentRole::Explorer, agent_id: "e1".into(), event: MidiEvent::note_on(0, 0, 60, 100) },
            Proposal { role: AgentRole::Conductor, agent_id: "c1".into(), event: MidiEvent::note_on(0, 0, 64, 100) },
        ];
        let events = r.resolve(proposals);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].pitch(), 64); // Conductor wins
    }

    // --- Ensemble tests ---

    #[test]
    fn ensemble_register_unregister() {
        let mut ens = Ensemble::new(TempoTracker::default(), KeySignature::c_major());
        let a = EnsembleAgent::new("p1".into(), AgentRole::Builder, 0, InstrumentPatch::piano());
        assert!(ens.register(a));
        assert_eq!(ens.agent_count(), 1);
        assert!(!ens.register(EnsembleAgent::new("p1".into(), AgentRole::Builder, 0, InstrumentPatch::piano())));
        assert!(ens.unregister("p1"));
        assert_eq!(ens.agent_count(), 0);
        assert!(!ens.unregister("nonexistent"));
    }

    #[test]
    fn ensemble_mute_solo() {
        let mut ens = Ensemble::new(TempoTracker::default(), KeySignature::c_major());
        ens.register(EnsembleAgent::new("a1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()));
        assert!(ens.set_muted("a1", true));
        assert!(ens.find_agent("a1").unwrap().muted);
        assert!(ens.set_solo("a1", true));
        assert!(ens.find_agent("a1").unwrap().solo);
        assert!(!ens.set_muted("nope", false));
    }

    #[test]
    fn ensemble_tick_synchronous() {
        let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::c_major());
        ens.register(EnsembleAgent::new("b1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()));
        let proposals = vec![
            Proposal { role: AgentRole::Builder, agent_id: "b1".into(), event: MidiEvent::note_on(0, 0, 60, 100) },
        ];
        let events = ens.tick(proposals);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].pitch(), 60);
        assert_eq!(ens.tempo().tick, 1); // advanced
    }

    #[test]
    fn ensemble_tick_muted_agent_ignored() {
        let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::c_major());
        ens.register(EnsembleAgent::new("b1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()).muted(true));
        let proposals = vec![
            Proposal { role: AgentRole::Builder, agent_id: "b1".into(), event: MidiEvent::note_on(0, 0, 60, 100) },
        ];
        let events = ens.tick(proposals);
        assert!(events.is_empty());
    }

    #[test]
    fn ensemble_solo_mode() {
        let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::c_major());
        ens.register(EnsembleAgent::new("b1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()));
        ens.register(EnsembleAgent::new("b2".into(), AgentRole::Builder, 1, InstrumentPatch::strings()).solo(true));
        let proposals = vec![
            Proposal { role: AgentRole::Builder, agent_id: "b1".into(), event: MidiEvent::note_on(0, 0, 60, 100) },
            Proposal { role: AgentRole::Builder, agent_id: "b2".into(), event: MidiEvent::note_on(0, 1, 64, 100) },
        ];
        let events = ens.tick(proposals);
        // Only b2 (solo) should play
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].pitch(), 64);
    }

    #[test]
    fn ensemble_mode_display() {
        assert_eq!(format!("{}", EnsembleMode::Synchronous), "Synchronous");
        assert_eq!(format!("{}", EnsembleMode::Improvisational), "Improvisational");
    }

    #[test]
    fn ensemble_reset() {
        let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::c_major());
        ens.register(EnsembleAgent::new("b1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()));
        let _ = ens.tick(vec![Proposal { role: AgentRole::Builder, agent_id: "b1".into(), event: MidiEvent::note_on(0, 0, 60, 100) }]);
        ens.reset();
        assert_eq!(ens.tempo().tick, 0);
        assert!(ens.stream().is_empty());
        // agents are preserved
        assert_eq!(ens.agent_count(), 1);
    }

    #[test]
    fn ensemble_asynchronous_mode_buffers_strong_beats() {
        let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::c_major());
        ens.set_mode(EnsembleMode::Asynchronous);
        ens.register(EnsembleAgent::new("b1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()));
        // tick 0 is a downbeat (strong) → no emit
        let proposals = vec![Proposal { role: AgentRole::Builder, agent_id: "b1".into(), event: MidiEvent::note_on(0, 0, 60, 100) }];
        let events = ens.tick(proposals);
        assert!(events.is_empty());
    }

    #[test]
    fn ensemble_improvisational_mode() {
        let mut ens = Ensemble::new(TempoTracker::new(120.0, 480), KeySignature::c_major());
        ens.set_mode(EnsembleMode::Improvisational);
        ens.register(EnsembleAgent::new("b1".into(), AgentRole::Builder, 0, InstrumentPatch::piano()));
        let proposals = vec![Proposal { role: AgentRole::Builder, agent_id: "b1".into(), event: MidiEvent::note_on(0, 0, 60, 100) }];
        let events = ens.tick(proposals);
        assert_eq!(events.len(), 1); // free mode: passes through
    }
}

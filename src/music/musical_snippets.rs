pub mod musical_snippets {
    use crate::music::instruments::Instrument::{Guitar, Organ};
    use crate::music::midi::NoteOff;
    use crate::music::midi::{MidiMessage, NoteOn};
    use crate::music::passage::Passage;

    pub const BEAT_LENGTH: u64 = 96;
    pub const BAR_LENGTH: u64 = 384;

    // note data for ep_downwards_2b.mid
    pub const EP_DOWNWARDS_2B: [MidiMessage; 8] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 50,
            velocity: 58,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 48,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 50,
            velocity: 64,
            delta_time: 374,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 66,
            delta_time: 10,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 51,
            velocity: 62,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 384,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 51,
            velocity: 64,
            delta_time: 0,
        }),
    ];
    // note data for gg_monotone.mid
    pub const GG_MONOTONE: [MidiMessage; 7] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 13,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 65,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 19,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 77,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 17,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 193,
        }),
    ];
    // note data for ep_chords_4b.mid
    pub const EP_CHORDS_4B: [MidiMessage; 13] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 36,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 40,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 40,
            velocity: 64,
            delta_time: 382,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 41,
            velocity: 100,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 41,
            velocity: 64,
            delta_time: 379,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 36,
            velocity: 64,
            delta_time: 3,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 38,
            velocity: 100,
            delta_time: 2,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 43,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 36,
            velocity: 100,
            delta_time: 384,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 38,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 43,
            velocity: 64,
            delta_time: 375,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 36,
            velocity: 64,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 0,
            velocity: 0,
            delta_time: 7,
        }),
    ];
    // note data for gg_turning.mid
    pub const GG_TURNING: [MidiMessage; 17] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 48,
            velocity: 90,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 48,
            velocity: 64,
            delta_time: 19,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 52,
            velocity: 72,
            delta_time: 29,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 52,
            velocity: 64,
            delta_time: 13,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 50,
            velocity: 66,
            delta_time: 11,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 50,
            velocity: 64,
            delta_time: 14,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 52,
            velocity: 58,
            delta_time: 58,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 52,
            velocity: 64,
            delta_time: 15,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 48,
            velocity: 90,
            delta_time: 33,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 48,
            velocity: 64,
            delta_time: 19,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 52,
            velocity: 72,
            delta_time: 29,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 52,
            velocity: 64,
            delta_time: 13,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 43,
            velocity: 80,
            delta_time: 35,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 43,
            velocity: 64,
            delta_time: 22,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 53,
            velocity: 72,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 53,
            velocity: 64,
            delta_time: 18,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 54,
        }),
    ];
    // note data for gg_expecting.mid
    pub const GG_EXPECTING: [MidiMessage; 11] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 60,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 45,
            velocity: 100,
            delta_time: 14,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 45,
            velocity: 64,
            delta_time: 52,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 38,
            velocity: 100,
            delta_time: 14,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 38,
            velocity: 64,
            delta_time: 56,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 43,
            velocity: 100,
            delta_time: 35,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 40,
            velocity: 100,
            delta_time: 51,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 43,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 40,
            velocity: 64,
            delta_time: 62,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 40,
        }),
    ];
    // note data for gg_falling.mid
    pub const GG_FALLING: [MidiMessage; 15] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 65,
            velocity: 58,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 65,
            velocity: 64,
            delta_time: 24,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 64,
            velocity: 45,
            delta_time: 72,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 59,
            velocity: 59,
            delta_time: 24,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 64,
            velocity: 64,
            delta_time: 5,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 59,
            velocity: 64,
            delta_time: 16,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 62,
            velocity: 44,
            delta_time: 147,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 57,
            velocity: 62,
            delta_time: 24,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 62,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 57,
            velocity: 64,
            delta_time: 22,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 60,
            velocity: 51,
            delta_time: 2,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 52,
            velocity: 62,
            delta_time: 24,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 60,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 52,
            velocity: 64,
            delta_time: 18,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 6,
        }),
    ];
    // note data for ep_chords_up_searching_1b.mid
    pub const EP_CHORDS_UP_SEARCHING_1B: [MidiMessage; 10] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 54,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 62,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 96,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 53,
            velocity: 47,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 53,
            velocity: 64,
            delta_time: 96,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 50,
            velocity: 54,
            delta_time: 47,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 55,
            velocity: 51,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 50,
            velocity: 64,
            delta_time: 138,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 55,
            velocity: 64,
            delta_time: 6,
        }),
    ];
    // note data for ep_downwards_4b.mid
    pub const EP_DOWNWARDS_4B: [MidiMessage; 16] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 50,
            velocity: 58,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 48,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 50,
            velocity: 64,
            delta_time: 374,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 49,
            velocity: 66,
            delta_time: 10,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 51,
            velocity: 62,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 42,
            velocity: 54,
            delta_time: 384,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 44,
            velocity: 52,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 51,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 49,
            velocity: 64,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 44,
            velocity: 64,
            delta_time: 381,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 46,
            velocity: 50,
            delta_time: 1,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 49,
            velocity: 50,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 46,
            velocity: 64,
            delta_time: 381,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 42,
            velocity: 64,
            delta_time: 3,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 49,
            velocity: 64,
            delta_time: 0,
        }),
    ];
    // note data for gg_hopeful.mid
    pub const GG_HOPEFUL: [MidiMessage; 11] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 15,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 45,
            velocity: 100,
            delta_time: 59,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 45,
            velocity: 64,
            delta_time: 8,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 38,
            velocity: 100,
            delta_time: 58,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 38,
            velocity: 64,
            delta_time: 12,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 43,
            velocity: 100,
            delta_time: 79,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 43,
            velocity: 64,
            delta_time: 7,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 41,
            velocity: 100,
            delta_time: 44,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 41,
            velocity: 64,
            delta_time: 18,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 84,
        }),
    ];
    // note data for gg_slight_rising.mid
    pub const GG_SLIGHT_RISING: [MidiMessage; 13] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 17,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 40,
            velocity: 100,
            delta_time: 68,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 40,
            velocity: 64,
            delta_time: 8,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 41,
            velocity: 100,
            delta_time: 75,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 41,
            velocity: 64,
            delta_time: 11,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 36,
            velocity: 100,
            delta_time: 88,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 36,
            velocity: 64,
            delta_time: 15,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 40,
            velocity: 100,
            delta_time: 36,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 40,
            velocity: 64,
            delta_time: 12,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 41,
            velocity: 100,
            delta_time: 37,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 41,
            velocity: 64,
            delta_time: 10,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 7,
        }),
    ];
    // note data for ep_what_2b.mid
    pub const EP_WHAT_2B: [MidiMessage; 5] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 52,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 55,
            velocity: 38,
            delta_time: 384,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 55,
            velocity: 64,
            delta_time: 1823,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 0,
            velocity: 0,
            delta_time: 96,
        }),
    ];
    // note data for drum_simple_cymbal.mid
    pub const DRUM_SIMPLE_CYMBAL: [MidiMessage; 11] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 48,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 9,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 87,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 9,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 49,
            velocity: 100,
            delta_time: 15,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 49,
            velocity: 64,
            delta_time: 10,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 62,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 10,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 86,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 10,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 0,
            velocity: 0,
            delta_time: 38,
        }),
    ];
    // note data for gg_rising.mid
    pub const GG_RISING: [MidiMessage; 17] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 48,
            velocity: 58,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 48,
            velocity: 64,
            delta_time: 24,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 53,
            velocity: 47,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 53,
            velocity: 64,
            delta_time: 14,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 50,
            velocity: 45,
            delta_time: 58,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 55,
            velocity: 59,
            delta_time: 24,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 50,
            velocity: 64,
            delta_time: 5,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 55,
            velocity: 64,
            delta_time: 16,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 52,
            velocity: 44,
            delta_time: 123,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 52,
            velocity: 64,
            delta_time: 24,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 57,
            velocity: 62,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 57,
            velocity: 64,
            delta_time: 22,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 53,
            velocity: 51,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 53,
            velocity: 64,
            delta_time: 24,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 0,
            note: 65,
            velocity: 62,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 65,
            velocity: 64,
            delta_time: 18,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 0,
            note: 0,
            velocity: 0,
            delta_time: 30,
        }),
    ];
    // note data for ep_chords_up_resolve_1b.mid
    pub const EP_CHORDS_UP_RESOLVE_1B: [MidiMessage; 12] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 53,
            velocity: 54,
            delta_time: 96,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 50,
            velocity: 66,
            delta_time: 47,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 53,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 50,
            velocity: 64,
            delta_time: 47,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 12,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 52,
            delta_time: 12,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 57,
            velocity: 61,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 166,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 57,
            velocity: 64,
            delta_time: 2,
        }),
    ];
    // note data for drum_simple.mid
    pub const DRUM_SIMPLE: [MidiMessage; 9] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 48,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 9,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 87,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 10,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 86,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 9,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 2,
            note: 46,
            velocity: 100,
            delta_time: 87,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 46,
            velocity: 64,
            delta_time: 10,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 2,
            note: 0,
            velocity: 0,
            delta_time: 38,
        }),
    ];
    // note data for ep_chords_downwards_1b.mid
    pub const EP_CHORDS_DOWNWARDS_1B: [MidiMessage; 10] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 40,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 45,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 38,
            velocity: 100,
            delta_time: 96,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 40,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 38,
            velocity: 64,
            delta_time: 95,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 36,
            velocity: 100,
            delta_time: 1,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 41,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 45,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 36,
            velocity: 64,
            delta_time: 192,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 41,
            velocity: 64,
            delta_time: 0,
        }),
    ];
    // note data for ep_chords_turning_1b.mid
    pub const EP_CHORDS_TURNING_1B: [MidiMessage; 18] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 34,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 58,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 50,
            velocity: 48,
            delta_time: 96,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 53,
            velocity: 41,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 43,
            velocity: 61,
            delta_time: 94,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 30,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 53,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 55,
            velocity: 48,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 50,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 43,
            velocity: 64,
            delta_time: 93,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 45,
            velocity: 84,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 53,
            velocity: 47,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 55,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 45,
            velocity: 64,
            delta_time: 92,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 53,
            velocity: 64,
            delta_time: 4,
        }),
    ];
    // note data for ep_slight_rising_chords_4b.mid
    pub const EP_SLIGHT_RISING_CHORDS_4B: [MidiMessage; 13] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 36,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 40,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 40,
            velocity: 64,
            delta_time: 382,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 41,
            velocity: 100,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 41,
            velocity: 64,
            delta_time: 379,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 36,
            velocity: 64,
            delta_time: 3,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 38,
            velocity: 100,
            delta_time: 2,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 43,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 36,
            velocity: 100,
            delta_time: 384,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 38,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 43,
            velocity: 64,
            delta_time: 375,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 36,
            velocity: 64,
            delta_time: 2,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 0,
            velocity: 0,
            delta_time: 7,
        }),
    ];
    // note data for ep_upwards_2b.mid
    pub const EP_UPWARDS_2B: [MidiMessage; 9] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 48,
            velocity: 52,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 52,
            velocity: 58,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 50,
            velocity: 39,
            delta_time: 384,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 52,
            velocity: 64,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 55,
            velocity: 38,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 48,
            velocity: 64,
            delta_time: 1,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 55,
            velocity: 64,
            delta_time: 383,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 50,
            velocity: 64,
            delta_time: 7,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 0,
            velocity: 0,
            delta_time: 377,
        }),
    ];

    pub const EP_MONO_2B: [MidiMessage; 4] = [
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 53,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOn(NoteOn {
            channel: 1,
            note: 57,
            velocity: 100,
            delta_time: 0,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 53,
            velocity: 64,
            delta_time: 384,
        }),
        MidiMessage::NoteOff(NoteOff {
            channel: 1,
            note: 57,
            velocity: 64,
            delta_time: 0,
        }),
    ];

    pub fn guitar_ending_sequence() -> Passage {
        Passage {
            instrument_association: Guitar,
            passage: vec![
                MidiMessage::NoteOn(NoteOn {
                    channel: 0,
                    note: 48,
                    velocity: 52,
                    delta_time: BEAT_LENGTH / 4,
                }),
                MidiMessage::NoteOff(NoteOff {
                    channel: 0,
                    note: 48,
                    velocity: 64,
                    delta_time: 0,
                }),
            ]
            .repeat(4 * 16),
        }
    }

    pub fn organ_starting_sequence() -> Vec<Vec<MidiMessage>> {
        vec![EP_MONO_2B.to_vec()]
    }

    pub fn guitar_sequences() -> Vec<Vec<MidiMessage>> {
        vec![
            GG_EXPECTING.to_vec(),
            GG_FALLING.to_vec(),
            GG_HOPEFUL.to_vec(),
            // GG_MONOTONE.to_vec(),
            GG_RISING.to_vec(),
            GG_SLIGHT_RISING.to_vec(),
            GG_TURNING.to_vec(),
        ]
    }

    pub fn organ_sequences(length: u64) -> Vec<Vec<MidiMessage>> {
        vec![
            EP_CHORDS_4B.to_vec(),
            EP_CHORDS_DOWNWARDS_1B.to_vec(),
            EP_CHORDS_TURNING_1B.to_vec(),
            EP_CHORDS_UP_RESOLVE_1B.to_vec(),
            EP_CHORDS_UP_SEARCHING_1B.to_vec(),
            EP_DOWNWARDS_2B.to_vec(),
            EP_DOWNWARDS_4B.to_vec(),
            EP_SLIGHT_RISING_CHORDS_4B.to_vec(),
            EP_UPWARDS_2B.to_vec(),
        ]
        .into_iter()
        .filter(|s| {
            let sequence_length = s.iter().map(|si| si.delta_time()).sum::<u64>();
            let sequence_bars = sequence_length / BAR_LENGTH;
            sequence_bars == length
        })
        .collect()
    }
}

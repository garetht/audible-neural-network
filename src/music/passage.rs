use std::time::{Duration, Instant};
use midir::MidiOutputConnection;
use crate::music::instruments::Instrument;
use crate::music::midi::MidiMessage;
use crate::music::midi::{NoteOff, NoteOn};


// A passage is a sequence of notes for a single channel
#[derive(Debug, Clone)]
pub struct Passage {
    pub instrument_association: Instrument,
    pub passage: Vec<MidiMessage>,
}

impl Passage {
    pub fn new(instrument_association: Instrument) -> Self {
        Self {
            instrument_association,
            passage: Vec::new(),
        }
    }

    pub fn merge(&self, other: &Passage) -> Passage {
        Passage {
            instrument_association: self.instrument_association.clone(),
            passage: self.passage.iter().chain(other.passage.iter()).cloned().collect(),
        }
    }

    pub fn total_length(&self) -> u64 {
        self.passage.iter().map(|message| message.delta_time()).sum()
    }

    pub fn channel(&self) -> u8 {
        let channels: Vec<u8> = self.passage.iter().filter_map( |message| {
            match message {
                MidiMessage::NoteOn(note_on) => Some(note_on.channel),
                MidiMessage::NoteOff(note_off) => Some(note_off.channel),
                _ => None,
            }
        }).collect();

        if let Some(channel) = channels.first() {
            *channel
        } else {
            0u8
        }
    }

    pub fn play_passage(&self, connection: &mut MidiOutputConnection, inverse_tempo: u64) {
        let mut scheduler = MidiScheduler {
            start_time: Instant::now()
        };
        scheduler.play_sequence(connection, inverse_tempo, &*self.passage);
    }
}

struct MidiScheduler {
    start_time: Instant,
}

impl MidiScheduler {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    fn ticks_to_duration(&self, inverse_tempo: u64, ticks: u64) -> Duration {
        let micros = ticks * inverse_tempo;
        Duration::from_micros(micros)
    }

    fn play_sequence(&mut self, connection: &mut MidiOutputConnection, inverse_tempo: u64, messages: &[MidiMessage]) {
        let mut next_message_scheduled_time = self.start_time;

        for message in messages {
            // Calculate when this message should play
            next_message_scheduled_time += self.ticks_to_duration(message.delta_time(), inverse_tempo);

            // Wait until it's time to send the message
            while Instant::now() < next_message_scheduled_time {
                // Busy-wait, but could yield thread if needed
                std::hint::spin_loop();
            }

            // Send the MIDI message
            connection.send(&message.to_message()).unwrap();
        }
    }
}

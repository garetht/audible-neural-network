use crate::music::instruments::Instrument::{Backprop, Drum, Guitar, Organ};
use crate::music::midi::{MidiMessage, NoteOff, NoteOn};
use crate::music::musical_snippets::musical_snippets::{
    DRUM_SIMPLE, EP_CHORDS_DOWNWARDS_1B, GG_FALLING,
};
use crate::music::passage::Passage;
use crate::music::port_finder::create_midi_output_connection;
use std::thread;

/**
 A score launches threads to play each passage simultaneously
**/
#[derive(Debug, Clone)]
pub struct Score {
    pub(crate) passages: Vec<Passage>,
}

impl Score {
    pub fn play(scores: Vec<Score>) {
        // play each score in turn, wait for it to complete
        // and then go to the next one
    }

    pub fn play_score(&self, inverse_tempo: u64) {
        let mut handles = vec![];
        let first_guitar = self
            .passages
            .iter()
            .find(|&passage| passage.instrument_association == Guitar);

        if let Some(passage) = first_guitar {
            let inner_passage = passage.clone();
            let guitar_playing_handle = thread::spawn(move || {
                // Create a new score instance for this thread
                println!("Thread 1 created score with length");
                let mut player = create_midi_output_connection().unwrap();
                inner_passage
                    .clone()
                    .play_passage(&mut player, inverse_tempo);
            });
            handles.push(guitar_playing_handle);
        }

        let first_organ = self
            .passages
            .iter()
            .find(|&passage| passage.instrument_association == Organ);
        if let Some(passage) = first_organ {
            let inner_passage = passage.clone();
            let organ_playing_handle = thread::spawn(move || {
                // Create a new score instance for this thread
                println!("Thread 2 created score with length");
                let mut player = create_midi_output_connection().unwrap();
                inner_passage.play_passage(&mut player, inverse_tempo);
            });
            handles.push(organ_playing_handle);
        }

        // let drum_passage = self
        //     .passages
        //     .iter()
        //     .filter_map(|passage| {
        //         if passage.instrument_association == Drum {
        //             Some(passage)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect::<Vec<&Passage>>();
        // let drum_playing_handle = thread::spawn(move || {
        //     // Create a new score instance for this thread
        //     println!("Thread 3 created score with length");
        //     let mut player = create_midi_output_connection().unwrap();
        //     drum_passage.play_score(&mut player, inverse_tempo);
        // });
        // handles.push(drum_playing_handle);

        let first_backprop = self
            .passages
            .iter()
            .find(|&passage| passage.instrument_association == Backprop);
        if let Some(passage) = first_backprop {
            let inner_passage = passage.clone();
            let organ_playing_handle = thread::spawn(move || {
                // Create a new score instance for this thread
                println!("Thread 4 created score with length");
                let mut player = create_midi_output_connection().unwrap();
                inner_passage.play_passage(&mut player, inverse_tempo);
            });
            handles.push(organ_playing_handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

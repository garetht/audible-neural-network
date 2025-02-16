use crate::music::control_knobs::control_knobs::{
    create_set_guitar_distortion_message, create_set_guitar_mosaic_message,
    create_set_organ_distortion_message, create_set_organ_mosaic_message,
};
use crate::music::instruments::Instrument;
use crate::music::instruments::Instrument::{Backprop, Guitar, Organ};
use crate::music::midi::{MidiMessage, NoteOff, NoteOn};
use crate::music::musical_snippets::musical_snippets::{guitar_sequences, organ_sequences, organ_starting_sequence};
use crate::music::passage::Passage;
use crate::music::score::Score;
use crate::network::parameters::ParameterDistance;
use rand::prelude::{SliceRandom, StdRng};

fn create_passage(
    rng: &mut StdRng,
    sequences: Vec<Vec<MidiMessage>>,
    instrument: Instrument,
    weights_distortion_creator: &dyn Fn() -> MidiMessage,
    bias_mosaic_creator: &dyn Fn() -> MidiMessage,
) -> Passage {
    let mut passage = vec![weights_distortion_creator(), bias_mosaic_creator()];

    let notes = sequences.choose(rng).unwrap();
    passage.extend(notes.iter());
    Passage {
        instrument_association: instrument,
        passage,
    }
}

pub fn generate_score(rng: &mut StdRng, parameter_distances: Vec<ParameterDistance>) -> Vec<Score> {
    let mut scores: Vec<Score> = vec![];

    // add a target length thingy here
    for i in 0..parameter_distances.len() {
        let mut scaling_factor = 1u64;
        if i > 5 {
            scaling_factor *= 2;
        }

        let parameter_distance = &parameter_distances[i];
        // generate first passage
        let hidden_layer_first_neuron_passage = create_guitar_passage(
            rng,
            1 * scaling_factor,
            parameter_distance.hidden_layer_first_neuron_bias_distance,
            parameter_distance.hidden_layer_first_neuron_weights_distance,
        );
        scores.push(Score {
            passages: vec![hidden_layer_first_neuron_passage],
        });

        let hidden_layer_second_neuron_passage = create_organ_passage(
            rng,
            1 * scaling_factor,
            parameter_distance.hidden_layer_second_neuron_bias_distance,
            parameter_distance.hidden_layer_second_neuron_weights_distance,
            i == 0
        );
        scores.push(Score {
            passages: vec![hidden_layer_second_neuron_passage],
        });

        let output_layer_first_hidden_contribution = create_guitar_passage(
            rng,
            2 * scaling_factor,
            parameter_distance.output_layer_bias_distance,
            parameter_distance.output_layer_weights_distance,
        );
        let output_layer_second_hidden_contribution = create_organ_passage(
            rng,
            2 * scaling_factor,
            parameter_distance.output_layer_bias_distance,
            parameter_distance.output_layer_weights_distance,
            false
        );

        scores.push(Score {
            passages: vec![
                output_layer_first_hidden_contribution,
                output_layer_second_hidden_contribution,
            ],
        });

        let backprop_passage = Passage {
            instrument_association: Backprop,
            passage: vec![
                MidiMessage::NoteOn(NoteOn {
                    channel: 3,
                    note: 60,
                    velocity: 127,
                    delta_time: 0,
                }),
                MidiMessage::NoteOff(NoteOff {
                    channel: 3,
                    note: 60,
                    velocity: 127,
                    delta_time: 384,
                }),
            ],
        };
        scores.push(Score {
            passages: vec![backprop_passage],
        });
    }

    scores
}

fn create_guitar_passage(
    rng: &mut StdRng,
    length: u64,
    bias_distance: f64,
    weights_distance: f64,
) -> Passage {
    let mut passages = vec![];
    for _ in 0..length {
        let passage = create_passage(
            rng,
            guitar_sequences(),
            Guitar,
            &move || create_set_guitar_mosaic_message(weights_distance as u8),
            &move || create_set_guitar_distortion_message(bias_distance as u8),
        );
        passages.push(passage);
    }

    passages
        .iter()
        .fold(Passage::new(Guitar), |acc, passage| acc.merge(passage))
}

fn create_organ_passage(
    rng: &mut StdRng,
    length: u64,
    bias_distance: f64,
    weights_distance: f64,
    force_starting_sequence: bool
) -> Passage {
    let mut organ_sequences = if force_starting_sequence {
        organ_starting_sequence()
    } else {
        organ_sequences(length)
    };

    organ_sequences.shuffle(rng);

    println!(
        "found {} organ sequences of length {}",
        organ_sequences.len(),
        length
    );

    create_passage(
        rng,
        organ_sequences,
        Organ,
        &move || create_set_organ_distortion_message(bias_distance as u8),
        &move || create_set_organ_mosaic_message(weights_distance as u8),
    )
}

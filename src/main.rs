mod music;
mod network;
mod training_data;

use crate::music::control_knobs::control_knobs::{create_activate_guitar_mosaic_message, create_activate_organ_mosaic_message, create_set_guitar_distortion_message, create_set_guitar_mosaic_message};
use crate::music::port_finder::create_midi_output_connection;
use crate::music::score_generation::generate_score;
use crate::network::layer_sizes::LayerSizes;
use crate::network::train_network;
use chrono::Local;
use ndarray_linalg::Scalar;
use plotters::prelude::LogScalable;
use rand::prelude::StdRng;
use rand::SeedableRng;
use crate::network::parameter_transfer::serialize_params;

fn main() {
    let mut connection = create_midi_output_connection().unwrap();
    connection
        .send(
            &create_activate_guitar_mosaic_message(true)
                .to_message()
                .to_vec(),
        )
        .unwrap();
    connection
        .send(
            &create_activate_organ_mosaic_message(true)
                .to_message()
                .to_vec(),
        )
        .unwrap();
    let date = Local::now();
    println!("{}", date.format("%Y-%m-%d][%H:%M:%S"));
    let mut rng = StdRng::seed_from_u64(101010u64);

    let training_results = train_network(&mut rng);

    dbg!(
        "{}",
        &training_results.final_parameters
    );

    let distances = training_results.calculate_distances();

    let selected_distances = distances.into_iter().step_by(18).collect();

    let scores = generate_score(&mut rng, selected_distances);

    // for i in 0..25 {
    //     println!("{}", 12000f64 * ((25.as_f64() - i.as_f64() + 1f64) / 25.as_f64()).sqrt());
    // }

    for i in 0..scores.len() {
        let score = &scores[i];
        let fraction = 1f64 - (i.as_f64() / scores.len().as_f64());
        println!("{}", fraction);
        let tempo = (12000f64 * (fraction.pow(5f64).clamp(0.04, 1.0))) as u64;
        println!("the tempo is {}", tempo);
        // if i > 10 {
        score.play_score(tempo);
        // }
        let date = Local::now();
        println!("{}", date.format("%Y-%m-%d][%H:%M:%S"));
    }
}

use crate::network::layer_sizes::LayerSizes;
use crate::network::network::NeuralNetwork;
use crate::network::parameters::{Parameters, TrainingResults};
use crate::training_data::load_training_data;
use ndarray::Array2;
use rand::prelude::StdRng;
use rand::SeedableRng;

pub mod layer_sizes;
pub mod network;
pub mod parameters;
pub mod parameter_transfer;

pub fn train_network(rng: &mut StdRng) -> TrainingResults {
    let training_data = load_training_data().unwrap();

    let layer_sizes = LayerSizes {
        input_layer_size: training_data.shape_x[0],
        hidden_layer_size: 2,
        output_layer_size: training_data.shape_y[0],
    };
    let number_of_samples = training_data.shape_x[1];

    let flat_training_features = training_data
        .features
        .iter()
        .flatten()
        .map(|f| *f)
        .collect();
    let training_features = Array2::from_shape_vec(
        (layer_sizes.input_layer_size, number_of_samples),
        flat_training_features,
    )
    .unwrap();

    let targets = Array2::from_shape_vec(
        (training_data.shape_y[0], number_of_samples),
        training_data.labels.iter().flatten().map(|f| *f).collect(),
    )
    .unwrap();

    let mut network = NeuralNetwork::new(rng, layer_sizes).unwrap();

    let training_results = network.train_model(200, 1.9, &training_features, &targets);

    training_results
}

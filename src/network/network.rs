use crate::LayerSizes;
use ndarray::Array2;
use rand::rngs::StdRng;
use rand::Rng;
use crate::network::parameters::{GradientUpdates, Parameters, PrecalculatedVectors, TrainingResults};

pub struct NeuralNetwork {
    parameters: Parameters,
}

impl NeuralNetwork {
    fn build_bias(number_of_rows: usize) -> Vec<f64> {
        vec![0.0; number_of_rows]
    }

    fn build_weights(
        rng: &mut StdRng,
        number_of_rows: usize,
        number_of_columns: usize,
    ) -> Vec<f64> {
        // loops: rows in outer loop, columns in inner loop
        let mut rows = vec![];
        for _ in 0..number_of_rows {
            let mut row = vec![];
            for _ in 0..number_of_columns {
                row.push(rng.gen::<f64>());
            }
            rows.push(row);
        }

        rows.into_iter().flatten().collect()
    }

    pub fn new(rng: &mut StdRng, layer_sizes: LayerSizes) -> anyhow::Result<Self> {
        let initialized_hidden_weights = Self::build_weights(
            rng,
            layer_sizes.hidden_layer_size,
            layer_sizes.input_layer_size,
        );

        let hidden_layer_weights = Array2::from_shape_vec(
            (layer_sizes.hidden_layer_size, layer_sizes.input_layer_size),
            initialized_hidden_weights,
        )?;

        let initialized_output_weights = Self::build_weights(
            rng,
            layer_sizes.output_layer_size,
            layer_sizes.hidden_layer_size,
        );
        let output_layer_weights = Array2::from_shape_vec(
            (layer_sizes.output_layer_size, layer_sizes.hidden_layer_size),
            initialized_output_weights,
        )?;

        let initialized_hidden_layer_bias = Self::build_bias(layer_sizes.hidden_layer_size);
        let hidden_layer_bias = Array2::from_shape_vec(
            (layer_sizes.hidden_layer_size, 1),
            initialized_hidden_layer_bias,
        )?;

        let initialized_output_layer_bias = Self::build_bias(layer_sizes.output_layer_size);
        let output_layer_bias = Array2::from_shape_vec(
            (layer_sizes.output_layer_size, 1),
            initialized_output_layer_bias,
        )?;

        Ok(NeuralNetwork {
            parameters: Parameters {
                hidden_layer_weights,
                output_layer_weights,
                hidden_layer_bias,
                output_layer_bias,
            },
        })
    }

    fn sigmoid(f: f64) -> f64 {
        1.0 / (1.0 + (-f).exp())
    }

    fn forward_propagate(
        training_samples: &Array2<f64>,
        parameters: &Parameters,
    ) -> (Array2<f64>, PrecalculatedVectors) {
        let Parameters {
            hidden_layer_weights,
            hidden_layer_bias,
            output_layer_weights,
            output_layer_bias,
        } = parameters;

        let hidden_layer_output = hidden_layer_weights.dot(training_samples) + hidden_layer_bias;
        let activated_hidden_layer_output = hidden_layer_output.mapv(|f| Self::sigmoid(f));

        let output_layer_output = output_layer_weights.dot(&activated_hidden_layer_output) + output_layer_bias;
        let activated_output_layer_output = output_layer_output.mapv(|f| Self::sigmoid(f));

        let precalculated_vectors = PrecalculatedVectors {
            hidden_layer_output,
            activated_hidden_layer_output,
            output_layer_output,
            activated_output_layer_output,
        };

        (precalculated_vectors.activated_output_layer_output.clone(), precalculated_vectors)
    }

    fn backwards_propagate(
        parameters: &Parameters,
        precalculated_vectors: &PrecalculatedVectors,
        training_samples: &Array2<f64>,
        true_outputs: &Array2<f64>,
    ) -> GradientUpdates {
        let Parameters {
            output_layer_weights,
            ..
        } = parameters;
        let samples: f64 = training_samples.shape()[1] as f64;
        let PrecalculatedVectors {
            activated_hidden_layer_output,
            activated_output_layer_output,
            ..
        } = precalculated_vectors;

        let d_z2 = activated_output_layer_output - true_outputs;
        let d_w2 = d_z2.dot(&activated_hidden_layer_output.t()) / samples;
        let d_b2 = d_z2.sum_axis(ndarray::Axis(1)).insert_axis(ndarray::Axis(1)) / samples;
        let d_z1 = output_layer_weights.t().dot(&d_z2)
            * activated_hidden_layer_output
            * (1.0 - activated_hidden_layer_output);
        let d_w1 = d_z1.dot(&training_samples.t()) / samples;
        let d_b1 = d_z1.sum_axis(ndarray::Axis(1)).insert_axis(ndarray::Axis(1)) / samples;

        let updates = GradientUpdates {
            hidden_layer_weight_updates: d_w1,
            hidden_layer_bias_updates: d_b1,
            output_layer_weight_updates: d_w2,
            output_layer_bias_updates: d_b2,
        };

        updates
    }

    fn update_parameters(
        parameters: &Parameters,
        gradient_updates: &GradientUpdates,
        learning_rate: f64,
    ) -> Parameters {
        let new_parameters = Parameters {
            hidden_layer_weights: &parameters.hidden_layer_weights
                - learning_rate * &gradient_updates.hidden_layer_weight_updates,
            hidden_layer_bias: &parameters.hidden_layer_bias
                - learning_rate * &gradient_updates.hidden_layer_bias_updates,
            output_layer_weights: &parameters.output_layer_weights
                - learning_rate * &gradient_updates.output_layer_weight_updates,
            output_layer_bias: &parameters.output_layer_bias
                - learning_rate * &gradient_updates.output_layer_bias_updates,
        };
        new_parameters
    }

    fn compute_cost(activated_output_layer_output: Array2<f64>, true_output: &Array2<f64>) -> f64 {
        let samples: f64 = true_output.shape()[1] as f64;
        let log_loss = 1f64 / samples * (
            (-1f64 * true_output)
            * activated_output_layer_output.ln()
            - (1f64 - true_output)
                * (1f64 - activated_output_layer_output.clone()).ln()
        ).sum();

        log_loss
    }

    pub fn train_model(
        &mut self,
        iterations: u32,
        learning_rate: f64,
        training_samples: &Array2<f64>,
        true_outputs: &Array2<f64>,
    ) -> TrainingResults {
        let mut iteration_parameters: Vec<Parameters> = vec![];
        for i in 0..iterations {
            let (new_predictions, precalculated_vectors) =
                NeuralNetwork::forward_propagate(&training_samples, &self.parameters);
            let gradient_updates = NeuralNetwork::backwards_propagate(
                &self.parameters,
                &precalculated_vectors,
                &training_samples,
                &true_outputs,
            );
            let new_parameters = NeuralNetwork::update_parameters(
                &self.parameters,
                &gradient_updates,
                learning_rate,
            );

            let old_parameters = std::mem::replace(&mut self.parameters, new_parameters);
            iteration_parameters.push(old_parameters);

            let cost = NeuralNetwork::compute_cost(new_predictions, &true_outputs);
            println!("Iteration: {}, {}", i, cost);
        }

        TrainingResults {
            final_parameters: self.parameters.clone(),
            iteration_parameters
        }
    }
}

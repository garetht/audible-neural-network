use std::cmp::max;
use ndarray::{Array, Array1, Array2, ArrayBase, ArrayView};
use ndarray_linalg::{Norm, Scalar};

#[derive(Clone, Debug)]
pub struct Parameters {
    pub hidden_layer_weights: Array2<f64>,
    pub output_layer_weights: Array2<f64>,
    pub hidden_layer_bias: Array2<f64>,
    pub output_layer_bias: Array2<f64>,
}

impl Parameters {
    pub fn debug_shapes(&self) {
        dbg!(self.hidden_layer_weights.shape());
        dbg!(self.output_layer_weights.shape());
        dbg!(self.hidden_layer_bias.shape());
        dbg!(self.output_layer_bias.shape());
    }
}

trait Normable {
    fn norm(&self) -> f64;
}

impl Normable for Array2<f64> {
    fn norm(&self) -> f64 {
        self.iter().map(|x| x.square()).sum::<f64>().sqrt()
    }
}

impl Normable for Array1<f64> {
    fn norm(&self) -> f64 {
        self.iter().map(|x| x.square()).sum::<f64>().sqrt()
    }
}

impl Normable for ArrayView<'_, f64, ndarray::Ix1> {
    fn norm(&self) -> f64 {
        self.iter().map(|x| x.square()).sum::<f64>().sqrt()
    }
}

#[derive(Clone, Debug)]
pub struct ParameterDistance {
    pub hidden_layer_first_neuron_weights_distance: f64,
    pub hidden_layer_second_neuron_weights_distance: f64,
    pub output_layer_weights_distance: f64,
    pub hidden_layer_first_neuron_bias_distance: f64,
    pub hidden_layer_second_neuron_bias_distance: f64,
    pub output_layer_bias_distance: f64,
}

impl ParameterDistance {
    pub fn new(
        total_distance: &ParameterDistance,
        final_parameters: &Parameters,
        intermediate_parameters: &Parameters,
    ) -> ParameterDistance {
        ParameterDistance {
            hidden_layer_first_neuron_weights_distance: (&final_parameters
                .hidden_layer_weights
                .row(0)
                - &intermediate_parameters.hidden_layer_weights.row(0))
                .norm()
                / total_distance.hidden_layer_first_neuron_weights_distance,
            hidden_layer_second_neuron_weights_distance: (&final_parameters
                .hidden_layer_weights
                .row(1)
                - &intermediate_parameters.hidden_layer_weights.row(1))
                .norm()
                / total_distance.hidden_layer_second_neuron_weights_distance,
            output_layer_weights_distance: (&final_parameters.output_layer_weights
                - &intermediate_parameters.output_layer_weights)
                .norm()
                / total_distance.output_layer_weights_distance,
            hidden_layer_first_neuron_bias_distance: (&final_parameters.hidden_layer_bias.row(0)
                - &intermediate_parameters.hidden_layer_bias.row(0))
                .norm()
                / total_distance.hidden_layer_first_neuron_bias_distance,
            hidden_layer_second_neuron_bias_distance: (&final_parameters.hidden_layer_bias.row(1)
                - &intermediate_parameters.hidden_layer_bias.row(1))
                .norm()
                / total_distance.hidden_layer_second_neuron_bias_distance,
            output_layer_bias_distance: (&final_parameters.output_layer_bias
                - &intermediate_parameters.output_layer_bias)
                .norm()
                / total_distance.output_layer_bias_distance,
        }
    }

    fn map(&self, f: fn(f64) -> f64) -> ParameterDistance {
        ParameterDistance {
            hidden_layer_first_neuron_weights_distance: f(
                self.hidden_layer_first_neuron_weights_distance
            ),
            hidden_layer_second_neuron_weights_distance: f(
                self.hidden_layer_second_neuron_weights_distance
            ),
            output_layer_weights_distance: f(self.output_layer_weights_distance),
            hidden_layer_first_neuron_bias_distance: f(self.hidden_layer_first_neuron_bias_distance),
            hidden_layer_second_neuron_bias_distance: f(
                self.hidden_layer_second_neuron_bias_distance
            ),
            output_layer_bias_distance: f(self.output_layer_bias_distance),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TrainingResults {
    pub final_parameters: Parameters,
    pub iteration_parameters: Vec<Parameters>,
}

impl TrainingResults {
    pub fn calculate_distances(&self) -> Vec<ParameterDistance> {
        let total_distance = ParameterDistance::new(
            &ParameterDistance {
                hidden_layer_first_neuron_weights_distance: 1.0,
                hidden_layer_second_neuron_weights_distance: 1.0,
                output_layer_weights_distance: 1.0,
                hidden_layer_first_neuron_bias_distance: 1.0,
                hidden_layer_second_neuron_bias_distance: 1.0,
                output_layer_bias_distance: 1.0,
            },
            &self.final_parameters,
            &self.iteration_parameters[0],
        );
        self.iteration_parameters
            .iter()
            .map(|x|
                ParameterDistance::new(&total_distance, &self.final_parameters, x)
                    .map(|distance| ((distance.clamp(0.0, 1.0).pow(3f64) * 126f64).floor()))
            )
            .collect()
    }
}

pub struct PrecalculatedVectors {
    // Z1
    pub hidden_layer_output: Array2<f64>,
    // A1
    pub activated_hidden_layer_output: Array2<f64>,
    // Z2
    pub output_layer_output: Array2<f64>,
    // A2 -> Y_hat
    pub activated_output_layer_output: Array2<f64>,
}

#[derive(Debug)]
pub struct GradientUpdates {
    pub hidden_layer_weight_updates: Array2<f64>,
    pub hidden_layer_bias_updates: Array2<f64>,
    pub output_layer_weight_updates: Array2<f64>,
    pub output_layer_bias_updates: Array2<f64>,
}

use crate::network::parameters::Parameters;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct NeuralNetworkParams {
    w1: Vec<Vec<f64>>,
    b1: Vec<Vec<f64>>,
    w2: Vec<Vec<f64>>,
    b2: Vec<Vec<f64>>,
}

pub fn serialize_params(parameters: Vec<Parameters>) {
    let mut all_params = vec![];
    for parameter in parameters {
        let serialize_params = NeuralNetworkParams {
            w1: parameter
                .hidden_layer_weights
                .rows()
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
            b1: parameter
                .hidden_layer_bias
                .rows()
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
            w2: parameter
                .output_layer_weights
                .rows()
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
            b2: parameter
                .output_layer_bias
                .rows()
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
        };
        all_params.push(serialize_params);
    }

    // Serialize to JSON string
    let json_string = serde_json::to_string(&all_params).unwrap();

    // Write to file
    let mut file = File::create("network_params.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingData {
    pub features: Vec<Vec<f64>>,
    pub labels: Vec<Vec<f64>>,
    pub shape_x: Vec<usize>,
    pub shape_y: Vec<usize>,
}

pub fn load_training_data() -> Result<TrainingData, Box<dyn std::error::Error>> {
    // Read the file
    let file_content = std::fs::read_to_string("./src/training_data.json")?;

    // Parse JSON
    let training_data: TrainingData = serde_json::from_str(&file_content)?;

    Ok(training_data)
}

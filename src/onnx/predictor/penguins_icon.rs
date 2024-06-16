use super::{base::ImageClassifierPredictor, Predictor};
use crate::{onnx::ONNXConfig, Result};
use image::DynamicImage;

pub struct PenguinsIconPredictor(ImageClassifierPredictor);

impl PenguinsIconPredictor {
    /// Create a new instance of the PenguinsIconPredictor
    pub async fn new(config: &ONNXConfig) -> Result<Self> {
        Ok(Self(
            ImageClassifierPredictor::new("penguins-icon.onnx", config).await?,
        ))
    }
}

impl Predictor for PenguinsIconPredictor {
    fn predict(&self, image: DynamicImage) -> Result<i32> {
        self.0.predict(image)
    }
}

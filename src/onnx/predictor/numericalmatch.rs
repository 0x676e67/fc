use super::{base::ImagePairClassifierPredictor, Predictor};
use crate::{onnx::ONNXConfig, Result};
use image::DynamicImage;

pub struct NumericalmatchPredictor(ImagePairClassifierPredictor);

impl NumericalmatchPredictor {
    /// Create a new instance of the NumericalmatchPredictor
    pub async fn new(config: &ONNXConfig) -> Result<Self> {
        Ok(Self(
            ImagePairClassifierPredictor::new("numericalmatch.onnx", config, false).await?,
        ))
    }
}

impl Predictor for NumericalmatchPredictor {
    fn predict(&self, image: DynamicImage) -> Result<i32> {
        self.0.predict(image)
    }
}

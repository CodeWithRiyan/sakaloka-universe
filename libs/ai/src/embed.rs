//! Text embedding using the local Burn model.

use crate::error::AiError;
use crate::model::EmbeddingModel;

/// Generates a fixed-dimension embedding vector for the given text.
///
/// The embedding is computed locally using the Burn inference engine.
/// No external API calls are made — Neptune is fully air-gapped from the internet.
///
/// # Errors
///
/// Returns [`AiError::InvalidInput`] if the input text is empty.
/// Returns [`AiError::ModelNotLoaded`] if the model file is missing.
/// Returns [`AiError::InferenceFailed`] if the forward pass fails.
///
/// # Examples
///
/// ```rust,no_run
/// use sakaloka_ai::{model::EmbeddingModel, embed::embed};
///
/// let model = EmbeddingModel::load("/models/sakaloka-embed.bin").unwrap();
/// let vector = embed("Hello Sakaloka", &model).unwrap();
/// assert_eq!(vector.len(), 384); // typical embedding dimension
/// ```
pub fn embed(text: &str, _model: &EmbeddingModel) -> Result<Vec<f32>, AiError> {
    if text.trim().is_empty() {
        return Err(AiError::InvalidInput("text cannot be empty".to_string()));
    }
    // TODO(Sprint 7): implement actual Burn forward pass.
    // Returning a stub zero-vector for scaffolding.
    Ok(vec![0.0_f32; 384])
}

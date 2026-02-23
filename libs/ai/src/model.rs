//! Burn model loading for Neptune (local AI inference).
//!
//! Models are loaded from a local file path — never downloaded at runtime.

use crate::error::AiError;

/// A loaded embedding model wrapper.
///
/// Wraps a Burn model ready for embedding inference.
pub struct EmbeddingModel {
    model_path: String,
}

impl EmbeddingModel {
    /// Loads an embedding model from the given file path.
    ///
    /// # Errors
    ///
    /// Returns [`AiError::ModelNotLoaded`] if the file does not exist
    /// at the specified path.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sakaloka_ai::model::EmbeddingModel;
    ///
    /// let model = EmbeddingModel::load("/models/sakaloka-embed.bin").unwrap();
    /// ```
    pub fn load(path: &str) -> Result<Self, AiError> {
        if !std::path::Path::new(path).exists() {
            return Err(AiError::ModelNotLoaded(path.to_string()));
        }
        Ok(Self {
            model_path: path.to_string(),
        })
    }

    /// Returns the file path of the loaded model.
    pub fn path(&self) -> &str {
        &self.model_path
    }
}

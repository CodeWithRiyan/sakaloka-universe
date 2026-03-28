//! Qdrant (Uranus) vector search client abstraction.
//!
//! Always request scoped keys from `sakaloka_secure::keys::vault::ApiKeyVault`.
//! Never use the Qdrant master API key in application code.
//!
//! The client connects to Qdrant over **gRPC** (default port 6334).
//! Make sure the gRPC port is exposed alongside the REST port in Docker.

use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, Filter, PointStruct, ScoredPoint, SearchPointsBuilder,
    UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::Qdrant;
use thiserror::Error;

/// Errors produced by the Qdrant client layer.
#[derive(Debug, Error)]
pub enum QdrantError {
    /// A transport or server error from the Qdrant gRPC driver.
    #[error("Qdrant error: {0}")]
    Client(Box<qdrant_client::QdrantError>),
}

impl From<qdrant_client::QdrantError> for QdrantError {
    fn from(err: qdrant_client::QdrantError) -> Self {
        Self::Client(Box::new(err))
    }
}

/// A thin wrapper around [`Qdrant`] enforcing scoped-key usage and
/// providing a simplified API for the Sakaloka embedding pipeline.
///
/// # Connection
///
/// ```text
/// let db = QdrantDb::connect("http://localhost:56334", Some("api-key"))?;
/// ```
///
/// # Collection lifecycle
///
/// ```text
/// db.ensure_collection("products", 384, Distance::Cosine).await?;
/// ```
pub struct QdrantDb {
    /// The underlying gRPC client.
    inner: Qdrant,
}

impl QdrantDb {
    /// Connect to a Qdrant instance using the given gRPC URL and optional
    /// scoped API key.
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] if the gRPC channel cannot be created.
    pub fn connect(url: &str, api_key: Option<&str>) -> Result<Self, QdrantError> {
        let mut builder = Qdrant::from_url(url);
        if let Some(key) = api_key {
            builder = builder.api_key(key);
        }
        let inner = builder
            .build()
            .map_err(|e| QdrantError::Client(Box::new(e)))?;
        tracing::info!(url = %url, "🔵 QdrantDb connected");
        Ok(Self { inner })
    }

    // ── Collection management ───────────────────────────────────

    /// Create a collection with a single dense-vector space.
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] on transport or server errors.
    /// Does **not** error if the collection already exists — use
    /// [`Self::collection_exists`] first if you need idempotency.
    pub async fn create_collection(
        &self,
        name: &str,
        dimension: u64,
        distance: Distance,
    ) -> Result<(), QdrantError> {
        self.inner
            .create_collection(
                CreateCollectionBuilder::new(name)
                    .vectors_config(VectorParamsBuilder::new(dimension, distance)),
            )
            .await?;
        tracing::info!(collection = %name, dim = dimension, "🔵 Collection created");
        Ok(())
    }

    /// Creates a collection only if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] on transport or server errors.
    pub async fn ensure_collection(
        &self,
        name: &str,
        dimension: u64,
        distance: Distance,
    ) -> Result<(), QdrantError> {
        if !self.collection_exists(name).await? {
            self.create_collection(name, dimension, distance).await?;
        }
        Ok(())
    }

    /// Check whether a collection exists.
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] on transport or server errors.
    pub async fn collection_exists(&self, name: &str) -> Result<bool, QdrantError> {
        Ok(self.inner.collection_exists(name).await?)
    }

    /// Delete a collection and all its data.
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] on transport or server errors.
    pub async fn delete_collection(&self, name: &str) -> Result<(), QdrantError> {
        self.inner.delete_collection(name).await?;
        tracing::info!(collection = %name, "🔵 Collection deleted");
        Ok(())
    }

    // ── Point operations ────────────────────────────────────────

    /// Upsert points into a collection.
    ///
    /// Each [`PointStruct`] contains an ID, a dense vector, and an
    /// arbitrary JSON payload.
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] on transport or server errors.
    pub async fn upsert(
        &self,
        collection: &str,
        points: Vec<PointStruct>,
    ) -> Result<(), QdrantError> {
        let count = points.len();
        self.inner
            .upsert_points(UpsertPointsBuilder::new(collection, points))
            .await?;
        tracing::debug!(collection = %collection, count, "🔵 Points upserted");
        Ok(())
    }

    /// Search for the `top_k` nearest neighbours in `collection`.
    ///
    /// An optional [`Filter`] can restrict the search space (e.g. by
    /// organisation ID or product category).
    ///
    /// # Errors
    ///
    /// Returns [`QdrantError::Client`] on transport or server errors.
    pub async fn search(
        &self,
        collection: &str,
        vector: Vec<f32>,
        top_k: u64,
        filter: Option<Filter>,
    ) -> Result<Vec<ScoredPoint>, QdrantError> {
        let mut builder = SearchPointsBuilder::new(collection, vector, top_k).with_payload(true);
        if let Some(f) = filter {
            builder = builder.filter(f);
        }
        let response = self.inner.search_points(builder).await?;
        Ok(response.result)
    }
}

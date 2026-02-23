//! Sakaloka-Universe global constants.

/// The name of the platform.
pub const PLATFORM_NAME: &str = "sakaloka-universe";

/// The current platform version.
pub const PLATFORM_VERSION: &str = env!("CARGO_PKG_VERSION");

/// JWT issuer identifier used in all token `iss` claims.
pub const JWT_ISSUER: &str = "sakaloka:iam";

/// The audience claim value for Earth (Loco.rs API).
pub const AUD_EARTH: &str = "sakaloka:earth";
/// The audience claim value for Jupiter (SurrealDB).
pub const AUD_JUPITER: &str = "sakaloka:jupiter";
/// The audience claim value for Saturn (Zenoh).
pub const AUD_SATURN: &str = "sakaloka:saturn";
/// The audience claim value for Uranus (Qdrant).
pub const AUD_URANUS: &str = "sakaloka:uranus";
/// The audience claim value for Neptune (Burn AI).
pub const AUD_NEPTUNE: &str = "sakaloka:neptune";

/// User JWT time-to-live in seconds (15 minutes).
pub const USER_TOKEN_TTL_SECS: u64 = 15 * 60;
/// Refresh token time-to-live in seconds (7 days).
pub const REFRESH_TOKEN_TTL_SECS: u64 = 7 * 24 * 60 * 60;
/// Service JWT time-to-live in seconds (5 minutes).
pub const SERVICE_TOKEN_TTL_SECS: u64 = 5 * 60;
/// Seconds before expiry at which service tokens are proactively refreshed.
pub const SERVICE_TOKEN_REFRESH_BUFFER_SECS: u64 = 60;

/// Zenoh topic namespace root.
pub const ZENOH_ROOT: &str = "sakaloka";

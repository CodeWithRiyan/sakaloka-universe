// ============================================================
// ⚡ Sakaloka-Universe — libs/hal
// Iron Curtain directives — no_std version
// ============================================================
#![no_std]
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # sakaloka-hal
//!
//! **Hardware Abstraction Layer (HAL) for Mercury (RTIC firmware).**
//!
//! This crate is `no_std` — it **must never** import `std`, `tokio`,
//! or any async runtime. RTIC provides its own task scheduler on bare metal.
//!
//! ## Design rules
//! - All peripheral access goes through traits defined here.
//! - RTIC internal `unsafe` (required by the framework) is isolated to
//!   `apps/firmware`. This crate has **zero** unsafe code.
//! - Shared domain types from `libs/core` are used via `no_std` feature flags.
//!
//! ## Sensor data flow
//! Mercury reads sensors → serialises to binary → sends via serial/USB bridge →
//! bridge process publishes to `sakaloka/mercury/sensor/{type}` on Saturn (Zenoh).

pub mod sensor;
pub mod serial;

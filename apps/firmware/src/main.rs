// ============================================================
// ⚡ Sakaloka-Universe — apps/firmware
// Iron Curtain directives — no_std entry point
// RTIC internal unsafe is confined to the #[app] macro expansion.
// ============================================================
#![no_std]
#![no_main]
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]

//! # sakaloka-firmware
//!
//! **Mercury** — RTIC 2.x bare-metal firmware for Sakaloka-Universe.
//!
//! Reads sensor data via hardware peripherals, serialises it, and
//! emits it over UART/USB. A host-side bridge process picks up
//! the serial stream and publishes to Saturn (Zenoh).
//!
//! ## Build
//! ```sh
//! cargo build --target thumbv7em-none-eabihf
//! ```
//!
//! ## Iron Curtain notes
//! `#![forbid(unsafe_code)]` is **not** applied here because the
//! `#[rtic::app]` macro expands to `unsafe` internally. All unsafe
//! is macro-generated — zero hand-written unsafe blocks exist.

use cortex_m_rt::entry;
use panic_halt as _;
use sakaloka_hal::sensor::SensorReading;
use sakaloka_hal::serial::write_reading;

/// Firmware entry point.
///
/// In production this will be replaced by the RTIC `#[app]` macro.
/// This stub satisfies the linker for `no_std` scaffolding purposes.
#[entry]
fn main() -> ! {
    let reading = SensorReading {
        sensor_id: 0,
        value_raw: 0,
        timestamp_ms: 0,
    };
    // Write reading to serial — the host bridge will relay it to Zenoh.
    write_reading(&reading);

    loop {
        cortex_m::asm::wfi();
    }
}

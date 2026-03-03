//! Serial output abstraction for Mercury → host bridge communication.

use crate::sensor::SensorReading;

/// Writes a [`SensorReading`] to the serial output.
///
/// In production this will drive a real UART peripheral via RTIC.
/// In unit tests this is a no-op stub so the HAL can be tested
/// without real hardware.
///
/// # Examples
///
/// ```rust
/// use sakaloka_hal::{sensor::SensorReading, serial::write_reading};
///
/// let r = SensorReading { sensor_id: 0, value_raw: 0, timestamp_ms: 0 };
/// write_reading(&r); // no-op in tests
/// ```
pub fn write_reading(_reading: &SensorReading) {
    // Stub: in production replaced by RTIC peripheral write.
    // Kept as no-op so the HAL compiles and tests pass on host.
}

//! Sensor abstractions for Mercury hardware peripherals.

/// A raw sensor reading captured by an RTIC task.
///
/// This is the unit of data that Mercury serialises and emits
/// over UART to the host-side Zenoh bridge.
#[derive(Debug, Clone, Copy)]
pub struct SensorReading {
    /// Unique sensor identifier (0–255).
    pub sensor_id: u8,
    /// Raw ADC or digital reading — interpretation is sensor-specific.
    pub value_raw: u32,
    /// Milliseconds since firmware boot (wraps at `u32::MAX`).
    pub timestamp_ms: u32,
}

impl SensorReading {
    /// Serialises the reading into a compact 9-byte little-endian frame.
    ///
    /// Wire format:
    /// ```text
    /// [sensor_id: u8][value_raw: u32 LE][timestamp_ms: u32 LE]
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_hal::sensor::SensorReading;
    ///
    /// let r = SensorReading { sensor_id: 1, value_raw: 42, timestamp_ms: 1000 };
    /// let bytes = r.to_bytes();
    /// assert_eq!(bytes[0], 1);
    /// ```
    pub fn to_bytes(&self) -> [u8; 9] {
        let mut out = [0u8; 9];
        out[0] = self.sensor_id;
        out[1..5].copy_from_slice(&self.value_raw.to_le_bytes());
        out[5..9].copy_from_slice(&self.timestamp_ms.to_le_bytes());
        out
    }
}

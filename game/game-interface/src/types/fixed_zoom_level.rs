use hiarc::Hiarc;
use math::math::vector::ffixed;
use serde::{Deserialize, Serialize};

/// A fixed zoom level where the allowed value range lies in
/// `[0.5 - 10.0]`.
#[derive(Debug, Hiarc, Default, Clone, Copy)]
pub struct FixedZoomLevel(ffixed);

impl Serialize for FixedZoomLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FixedZoomLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = ffixed::deserialize(deserializer)?;
        Ok(Self(
            val.clamp(ffixed::from_num(0.5), ffixed::from_num(10.0)),
        ))
    }
}

impl FixedZoomLevel {
    /// The resulting value is automatically clamped to `[0.5 - 10.0]`.
    pub fn new_lossy(mut val: f64) -> Self {
        if val.is_infinite() {
            val = 10.0;
        }
        if val.is_nan() {
            val = 1.0;
        }
        val = val.clamp(0.5, 10.0);
        Self(ffixed::from_num(val))
    }

    pub fn as_f64(self) -> f64 {
        self.0.to_num()
    }
}

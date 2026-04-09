use serde::{Deserialize, Serialize};

/// A geographic coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Point {
    /// The latitude of a coordinate in degrees north of the equator (-90 to 90).
    pub lat: f64,
    /// The longitude of a coordinate in degrees east of the prime meridian (-180 to 180).
    pub lon: f64,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

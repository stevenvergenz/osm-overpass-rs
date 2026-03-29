use serde::{Deserialize, Serialize};
use crate::{Bbox, ElementCommon, Point};

/// A way is one of the fundamental elements of the map.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Way)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Way {
    #[serde(flatten)]
    pub common: ElementCommon,
    pub nodes: Vec<i64>,
    pub geometry: Option<Vec<Point>>,
    pub bounds: Option<Bbox>,
    pub center: Option<Point>,
}

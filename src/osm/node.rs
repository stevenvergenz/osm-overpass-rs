use serde::{Deserialize, Serialize};
use crate::{ElementCommon, Point};

/// A node is one of the core elements in the OpenStreetMap data model.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Node {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(flatten)]
    pub point: Point,
}
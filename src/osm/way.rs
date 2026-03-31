use crate::{Bbox, ElementId, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A way is one of the fundamental elements of the map.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Way)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Way {
    #[serde(deserialize_with = "crate::de::IdVisitor::parse_way")]
    pub id: ElementId,
    pub tags: HashMap<String, String>,
    #[serde(deserialize_with = "crate::de::parse_nodes")]
    pub nodes: Vec<ElementId>,
    pub geometry: Option<Vec<Point>>,
    pub bounds: Option<Bbox>,
    pub center: Option<Point>,
}

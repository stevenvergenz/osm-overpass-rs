use crate::{ElementId, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node is one of the core elements in the OpenStreetMap data model.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Node {
    #[serde(deserialize_with = "crate::de::IdVisitor::parse_node")]
    pub id: ElementId,
    pub tags: HashMap<String, String>,

    #[serde(flatten)]
    pub point: Point,
}

use crate::{ElementId, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node is one of the core elements in the OpenStreetMap data model.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Node {
    /// The unique identifer of this element.
    #[serde(deserialize_with = "crate::de::IdVisitor::parse_node")]
    pub id: ElementId,

    /// The tags on this element.
    pub tags: HashMap<String, String>,

    /// The geographic coordinates of this node.
    #[serde(flatten)]
    pub point: Point,
}

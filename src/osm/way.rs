use crate::{Bbox, ElementId, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One of the fundamental elements of the map, composed of a sequence of nodes.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Way)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Way {
    /// The id of this element.
    #[serde(deserialize_with = "crate::de::IdVisitor::parse_way")]
    pub id: ElementId,

    /// This element's tags.
    pub tags: HashMap<String, String>,

    /// The ids of this way's nodes in order.
    #[serde(deserialize_with = "crate::de::parse_nodes")]
    pub nodes: Vec<ElementId>,

    /// If [QueryGeometry::Geometry], the coordinates of the way path.
    pub geometry: Option<Vec<Point>>,

    /// If [QueryGeometry::Geometry], the bounds of the way.
    pub bounds: Option<Bbox>,

    /// If [QueryGeometry::Center], the center point of the way's bounding box.
    pub center: Option<Point>,
}

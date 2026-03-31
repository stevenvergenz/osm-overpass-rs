use crate::{Bbox, ElementId, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Relations are structured collections of objects - nodes, ways, and other relations.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Relation {
    #[serde(deserialize_with = "crate::de::IdVisitor::parse_relation")]
    pub id: ElementId,
    pub tags: HashMap<String, String>,
    pub members: Vec<RelationMember>,
    pub bounds: Option<Bbox>,
    pub center: Option<Point>,
}

/// A reference to another [Element] from the owning [Relation].
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RelationMember {
    #[serde(flatten)]
    pub id: ElementId,

    /// The role of this element in a relation, if any.
    #[serde(deserialize_with = "crate::de::skip_empty")]
    pub role: Option<String>,

    pub geometry: Option<Vec<Point>>,
}

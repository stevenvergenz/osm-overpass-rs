use crate::{Bbox, ElementId, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(doc)]
use crate::QueryGeometry;

/// Relations are structured collections of objects - nodes, ways, and other relations.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Relation {
    /// The unique identifier of this element.
    #[serde(deserialize_with = "crate::de::IdVisitor::parse_relation")]
    pub id: ElementId,

    /// The tags on this element.
    pub tags: HashMap<String, String>,

    /// The relation's connections to other elements.
    pub members: Vec<RelationMember>,

    /// If [QueryGeometry::Geometry] or [QueryGeometry::Bbox], the bounds containing this relation's member geometry.
    pub bounds: Option<Bbox>,

    /// If [QueryGeometry::Center], the center point of this relation's member geometry.
    pub center: Option<Point>,
}

/// A reference to another [Element] from the owning [Relation].
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RelationMember {
    /// The id of the related element.
    #[serde(flatten)]
    pub id: ElementId,

    /// The role of this element in a relation, if any.
    #[serde(deserialize_with = "crate::de::skip_empty")]
    pub role: Option<String>,

    /// If [QueryGeometry::Geometry] is specified, this may contain point/path data.
    pub geometry: Option<Vec<Point>>,
}

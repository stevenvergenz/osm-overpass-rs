use std::collections::HashMap;

use crate::{Node, Relation, Way};
use serde::{Deserialize, Serialize};

/// The basic component of OpenStreetMap's data model. Comes in three variants: [Node], [Way], and [Relation].
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Elements)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

/// The identifier of an [Element]: the type and the numeric identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "ref")]
pub enum ElementId {
    Node(i64),
    Way(i64),
    Relation(i64),
}

impl ElementId {
    /// The numeric value of the id.
    pub fn value(&self) -> i64 {
        match self {
            Self::Node(id) => *id,
            Self::Way(id) => *id,
            Self::Relation(id) => *id,
        }
    }
}

impl Element {
    /// True if the element is a node.
    pub fn is_node(&self) -> bool {
        matches!(self, Self::Node(_))
    }

    /// True if the element is a way.
    pub fn is_way(&self) -> bool {
        matches!(self, Self::Way(_))
    }

    /// True if the element is a relation.
    pub fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(_))
    }

    /// The unique identifier of this element.
    pub fn id(&self) -> ElementId {
        match self {
            Self::Node(n) => n.id,
            Self::Way(w) => w.id,
            Self::Relation(r) => r.id,
        }
    }

    /// The set of tags on the element.
    pub fn tags(&self) -> &HashMap<String, String> {
        match self {
            Self::Node(n) => &n.tags,
            Self::Way(w) => &w.tags,
            Self::Relation(r) => &r.tags,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use super::*;

    #[test]
    fn id() {
        let a = ElementId::Node(123);
        let str = serde_json::to_string(&a).unwrap();
        assert_eq!(&str, r#"{"type":"node","ref":123}"#);
        let b: ElementId = serde_json::from_str(&str).unwrap();
        assert_eq!(a, b);
    }

    #[tokio::test]
    async fn geom() -> Result<(), Box<dyn std::error::Error>> {
        let q: Query = SetBuilder::union([
            SetBuilder::nodes().with_id(3359850618) ,
            SetBuilder::ways().with_id(12903132),
            SetBuilder::relations().with_id(19745997),
        ])
        .to_output()
        .geometry(QueryGeometry::Geometry)
        .to_query()
        .search_bbox(Bbox { north: 47.667, south: 47.553, east: -122.201, west: -122.461 })
        .into();
        println!("{}", q.to_oql());
        let res = OverpassServer::default().evaluate(&q).await?;

        let way = res
            .elements
            .iter()
            .find_map(|e| {
                if let Element::Way(w) = e {
                    Some(w)
                } else {
                    None
                }
            })
            .unwrap();
        assert_ne!(way.bounds, None);
        assert_eq!(way.geometry.as_ref().map(|w| w.len()), Some(32));

        let rel = res
            .elements
            .iter()
            .find_map(|e| {
                if let Element::Relation(r) = e {
                    Some(r)
                } else {
                    None
                }
            })
            .unwrap();
        assert_ne!(rel.bounds, None);
        assert_ne!(rel.members[0].geometry, None);

        Ok(())
    }
}

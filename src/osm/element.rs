use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{Node, Way, Relation, Bbox, Point};

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

/// Data common to all variants of [Element]
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ElementCommon {
    pub id: i64,
    pub tags: HashMap<String, String>,
}

/// The identifier of an [Element], independent of the specific variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "ref")]
pub enum ElementId {
    Node(i64),
    Way(i64),
    Relation(i64),
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

    /// The [ElementId] of this element.
    pub fn id(&self) -> ElementId {
        match self {
            Self::Node(n) => ElementId::Node(n.common.id),
            Self::Way(w) => ElementId::Way(w.common.id),
            Self::Relation(r) => ElementId::Relation(r.common.id),
        }
    }

    /// The value of this element's tag with the given name, if one exists.
    pub fn tag(&self, name: &str) -> Option<&str> {
        let tags = match self {
            Self::Node(n) => &n.common.tags,
            Self::Way(w) => &w.common.tags,
            Self::Relation(r) => &r.common.tags,
        };
        tags.get(name).map(|s| s.as_str())
    }

    /// An iterator of tag values on this element, composed of key/value tuples.
    pub fn tags(&self) -> impl ExactSizeIterator<Item = (&str, &str)> {
        let tags = match self {
            Self::Node(n) => &n.common.tags,
            Self::Way(w) => &w.common.tags,
            Self::Relation(r) => &r.common.tags,
        };
        tags.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub fn bounds(&self) -> Option<Bbox> {
        match self {
            Self::Node(n) => Some(n.point.into()),
            Self::Way(w) => w.bounds,
            Self::Relation(r) => r.bounds,
        }
    }

    pub fn center(&self) -> Option<Point> {
        match self {
            Self::Node(n) => Some(n.point),
            Self::Way(w) => w.center,
            Self::Relation(r) => r.center,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Overpass, OverpassServer, Query, QueryGeometry, SetBuilder, ToQuery};

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
            SetBuilder::nodes().with_id(3359850618),
            SetBuilder::ways().with_id(12903132),
            SetBuilder::relations().with_id(19745997),
        ])
        .to_query()
        .geometry(QueryGeometry::Geometry)
        .into();
        
        let res = OverpassServer::default().evaluate(&q).await?;
        let node = res.elements.iter().find(|e| e.is_node()).unwrap();
        assert_eq!(node.bounds(), None);

        let way = res.elements.iter().find_map(|e| {
            if let Element::Way(w) = e {
                Some(w)
            } else {
                None
            }
        }).unwrap();
        assert_ne!(way.bounds, None);
        assert_eq!(way.geometry.as_ref().map(|w| w.len()), Some(32));

        let rel = res.elements.iter().find_map(|e| {
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

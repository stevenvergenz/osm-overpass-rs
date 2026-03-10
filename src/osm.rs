use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};

/// The basic component of OpenStreetMap's data model. Comes in three variants: [Node], [Way], and [Relation].
/// [wiki](https://wiki.openstreetmap.org/wiki/Elements)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase", tag="type")]
pub enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

/// The identifier of an [Element], independent of the specific variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "ref")]
pub enum ElementId {
    Node(i64),
    Way(i64),
    Relation(i64),
}

impl Element {
    /// The [ElementId] of this element.
    pub fn id(&self) -> ElementId {
        match self {
            Self::Node(n) => ElementId::Node(n.id),
            Self::Way(w) => ElementId::Way(w.id),
            Self::Relation(r) => ElementId::Relation(r.id),
        }
    }

    /// The value of this element's tag with the given name, if one exists.
    pub fn tag(&self, name: &str) -> Option<&str> {
        let tags = match self {
            Self::Node(n) => &n.tags,
            Self::Way(w) => &w.tags,
            Self::Relation(r) => &r.tags,
        };
        tags.get(name).map(|s| s.as_str())
    }

    /// An iterator of tag values on this element, composed of key/value tuples.
    pub fn tags(&self) -> impl ExactSizeIterator<Item=(&str, &str)> {
        let tags = match self {
            Self::Node(n) => &n.tags,
            Self::Way(w) => &w.tags,
            Self::Relation(r) => &r.tags,
        };
        tags.iter().map(|(k,v)| (k.as_str(), v.as_str()))
    }
}

/// A node is one of the core elements in the OpenStreetMap data model.
/// [wiki](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: HashMap<String, String>,
}

/// A way is one of the fundamental elements of the map.
/// [wiki](https://wiki.openstreetmap.org/wiki/Way)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Way {
    pub id: i64,
    pub tags: HashMap<String, String>,
    pub nodes: Vec<i64>,
}

/// Relations are structured collections of objects - nodes, ways, and other relations.
/// [wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub id: i64,
    pub tags: HashMap<String, String>,
    pub members: Vec<RelationMember>,
}

/// A reference to another [Node], [Way], or [Relation] from this relation, with an optional role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationMember {
    #[serde(flatten)]
    pub id: ElementId,
    #[serde(deserialize_with = "skip_empty")]
    pub role: Option<String>,
}

fn skip_empty<'de, D>(deserializer: D) -> Result<Option<String>, D::Error> where D: Deserializer<'de> {
    deserializer.deserialize_string(OptionalStringVisitor)
}

struct OptionalStringVisitor;
impl<'de> Visitor<'de> for OptionalStringVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        if v.is_empty() {
            Ok(None)
        } else {
            Ok(Some(v))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn id() {
        let a = ElementId::Node(123);
        let str = serde_json::to_string(&a).unwrap();
        assert_eq!(&str, r#"{"type":"node","ref":123}"#);
        let b: ElementId = serde_json::from_str(&str).unwrap();
        assert_eq!(a, b);
    }
}

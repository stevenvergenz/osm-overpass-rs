use serde::{Deserialize, Serialize};

mod node;
pub use node::*;

mod way;
pub use way::*;

mod relation;
pub use relation::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase", tag="type")]
pub enum OsmElement {
    Node(OsmNode),
    Way(OsmWay),
    Relation(OsmRelation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "ref")]
pub enum OsmElementId {
    Node(i64),
    Way(i64),
    Relation(i64),
}

impl OsmElement {
    pub fn id(&self) -> OsmElementId {
        match self {
            Self::Node(n) => OsmElementId::Node(n.id),
            Self::Way(w) => OsmElementId::Way(w.id),
            Self::Relation(r) => OsmElementId::Relation(r.id),
        }
    }

    pub fn tag(&self, name: &str) -> Option<&str> {
        let tags = match self {
            Self::Node(n) => &n.tags,
            Self::Way(w) => &w.tags,
            Self::Relation(r) => &r.tags,
        };
        tags.get(name).map(|s| s.as_str())
    }

    pub fn tags(&self) -> impl ExactSizeIterator<Item=(&str, &str)> {
        let tags = match self {
            Self::Node(n) => &n.tags,
            Self::Way(w) => &w.tags,
            Self::Relation(r) => &r.tags,
        };
        tags.iter().map(|(k,v)| (k.as_str(), v.as_str()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn id() {
        let a = OsmElementId::Node(123);
        let str = serde_json::to_string(&a).unwrap();
        assert_eq!(&str, r#"{"type":"node","ref":123}"#);
        let b: OsmElementId = serde_json::from_str(&str).unwrap();
        assert_eq!(a, b);
    }
}

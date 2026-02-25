use serde::{Deserialize, Serialize};

mod node;
pub use node::*;

mod way;
pub use way::*;

mod relation;
pub use relation::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase", tag="type")]
pub enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "ref")]
pub enum ElementId {
    Node(i64),
    Way(i64),
    Relation(i64),
}

impl Element {
    pub fn id(&self) -> ElementId {
        match self {
            Self::Node(n) => ElementId::Node(n.id),
            Self::Way(w) => ElementId::Way(w.id),
            Self::Relation(r) => ElementId::Relation(r.id),
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
        let a = ElementId::Node(123);
        let str = serde_json::to_string(&a).unwrap();
        assert_eq!(&str, r#"{"type":"node","ref":123}"#);
        let b: ElementId = serde_json::from_str(&str).unwrap();
        assert_eq!(a, b);
    }
}

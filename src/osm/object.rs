use serde::{Deserialize, Serialize};
use crate::OsmNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase", tag="type")]
pub enum OsmObject {
    Node(OsmNode),
}

impl OsmObject {
    pub fn id(&self) -> i64 {
        match self {
            Self::Node(n) => n.id,
        }
    }

    pub fn tag(&self, name: &str) -> Option<&str> {
        let tags = match self {
            Self::Node(n) => &n.tags,
        };
        tags.get(name).map(|s| s.as_str())
    }

    pub fn tags(&self) -> impl Iterator<Item=(&str, &str)> {
        let tags = match self {
            Self::Node(n) => &n.tags,
        };
        tags.iter().map(|(k,v)| (k.as_str(), v.as_str()))
    }
}

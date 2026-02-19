use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::OsmElementId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationMember {
    #[serde(flatten)]
    pub id: OsmElementId,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsmRelation {
    pub id: i64,
    pub tags: HashMap<String, String>,
    pub members: Vec<RelationMember>,
}

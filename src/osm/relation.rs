use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::ElementId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationMember {
    #[serde(flatten)]
    pub id: ElementId,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub id: i64,
    pub tags: HashMap<String, String>,
    pub members: Vec<RelationMember>,
}

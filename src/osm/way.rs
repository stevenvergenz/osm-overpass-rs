use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OsmWay {
    pub id: i64,
    pub tags: HashMap<String, String>,
    pub nodes: Vec<i64>,
}


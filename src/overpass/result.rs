use crate::{Element, ElementId};
use serde::Deserialize;
use std::collections::HashMap;

/// The data returned from an [Overpass] [Query] evaluation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(try_from = "super::count::PreResult")]
pub struct OverpassResult {
    /// The [Element]s in the query set.
    pub elements: Vec<Element>,

    /// If [QueryVerbosity::Count](crate::QueryVerbosity::Count), the number of each type of element that would have
    /// been returned.
    pub counts: Option<ResultCount>,

    /// Miscellaneous information provided by the API server, such as the API version and a timestamp.
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>,
}

/// Statistics for the result of a [Query].
#[derive(Debug, Clone, PartialEq)]
pub struct ResultCount {
    /// The number of nodes in the result set.
    pub nodes: u32,
    /// The number of ways in the result set.
    pub ways: u32,
    /// The number of relations in the result set.
    pub relations: u32,
    /// The total number of elements in the result set.
    pub total: u32,
}

impl OverpassResult {
    /// Index the [elements](Self::elements) by their [id](Element::id) and return the lookup map.
    pub fn elements_by_id(&self) -> HashMap<ElementId, &Element> {
        self.elements.iter().map(|e| (e.id(), e)).collect()
    }
}

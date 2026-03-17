use crate::{Element, OverpassQLError, Query};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FResult},
};

mod server;
pub use server::*;

/// An error returned when a request to evaluate a [Query] fails.
#[derive(Debug)]
pub enum OverpassError {
    /// There was an error serializing the query.
    Query(OverpassQLError),
    /// There was an error communicating with the Overpass server.
    Request(reqwest::Error),
    /// There was an error parsing the response from the Overpass server.
    Parse(serde_json::Error),
    /// An unknown error occurred.
    Other(String),
}
impl Display for OverpassError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::Query(e) => write!(f, "{e}"),
            Self::Request(e) => write!(f, "{e}"),
            Self::Parse(e) => write!(f, "Deserialization error: {e}"),
            Self::Other(e) => write!(f, "Error from API provider: {e}"),
        }
    }
}
impl std::error::Error for OverpassError {}

/// The data returned from an [Overpass] [Query] evaluation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OverpassResult {
    /// The [Element]s in the query set.
    pub elements: Vec<Element>,

    /// Miscellaneous information provided by the API server, such as the API version and a timestamp.
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>,
}

/// Can retrieve [Element] data from OpenStreetMap that matches the provided [Query] set.
pub trait Overpass {
    /// An async method that evaluates a [Query] against the map database and returns the
    /// resulting [Element]s.
    fn evaluate(
        &self,
        query: &Query<'_>,
    ) -> impl std::future::Future<Output = Result<OverpassResult, OverpassError>> + Send;
}

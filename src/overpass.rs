use std::{
    fmt::{Display, Formatter, Result as FResult},
};
use serde::Deserialize;
use crate::{Element, OverpassQLError, Query};

mod server;
pub use server::*;

#[derive(Debug)]
pub enum OverpassError {
    Query(OverpassQLError),
    Request(reqwest::Error),
    Parse(serde_json::Error),
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OverpassResult {
    elements: Vec<Element>,
    //#[serde(flatten)]
    //meta: HashMap<String, String>,
}

pub trait Overpass {
    fn evaluate(&self, query: &Query) -> impl std::future::Future<Output = Result<OverpassResult, OverpassError>> + Send;
}

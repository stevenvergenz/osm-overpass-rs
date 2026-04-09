use crate::{Element, Node, OverpassResult, Relation, ResultCount, Way};
use serde::Deserialize;
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug, Deserialize)]
pub(crate) struct PreResult<'a> {
    #[serde(borrow)]
    elements: Vec<PreElement<'a>>,
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub(crate) enum PreElement<'a> {
    Node(Node),
    Way(Way),
    Relation(Relation),
    Count {
        #[serde(borrow)]
        tags: PreCount<'a>,
    },
}
impl From<PreElement<'_>> for Element {
    fn from(value: PreElement<'_>) -> Self {
        match value {
            PreElement::Relation(r) => Self::Relation(r),
            PreElement::Way(w) => Self::Way(w),
            PreElement::Node(n) => Self::Node(n),
            _ => panic!("Cannot convert count object to element"),
        }
    }
}
#[derive(Debug, Deserialize)]
pub(crate) struct PreCount<'a> {
    nodes: &'a str,
    ways: &'a str,
    relations: &'a str,
    total: &'a str,
}

impl TryFrom<PreResult<'_>> for OverpassResult {
    type Error = ParseIntError;
    fn try_from(value: PreResult<'_>) -> Result<Self, Self::Error> {
        if let Some(PreElement::Count { tags }) = value.elements.get(0) {
            Ok(Self {
                elements: vec![],
                counts: Some(ResultCount {
                    nodes: u32::from_str_radix(tags.nodes, 10)?,
                    ways: u32::from_str_radix(tags.ways, 10)?,
                    relations: u32::from_str_radix(tags.relations, 10)?,
                    total: u32::from_str_radix(tags.total, 10)?,
                }),
                other_fields: value.other_fields,
            })
        } else {
            Ok(Self {
                elements: value
                    .elements
                    .into_iter()
                    .map(|e| e.into())
                    .collect(),
                counts: None,
                other_fields: value.other_fields,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Bbox, Overpass, OverpassServer, QueryVerbosity, SetBuilder,
        SetBuilderCommon,
    };

    use super::*;

    #[tokio::test]
    async fn count() {
        let r = OverpassServer::default()
            .evaluate(
                SetBuilder::ways()
                    .with_id(12903132)
                    .to_output()
                    .verbosity(QueryVerbosity::Count)
                    .to_query()
                    .search_bbox(Bbox {
                        north: 47.667,
                        south: 47.553,
                        east: -122.201,
                        west: -122.461,
                    })
                    .as_ref(),
            )
            .await
            .expect("Query evaluation error");

        assert_eq!(
            r.counts,
            Some(ResultCount {
                nodes: 0,
                ways: 1,
                relations: 0,
                total: 1
            })
        );
    }
}

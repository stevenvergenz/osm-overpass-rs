use std::{borrow::Cow, sync::LazyLock};
use crate::{Overpass, OverpassError, OverpassQLUnnamed, OverpassResult, Query};
use reqwest::Client;

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

#[derive(Debug)]
pub struct OverpassServer {
    pub client: Cow<'static, Client>,
    pub url: String,
}

impl Default for OverpassServer {
    fn default() -> Self {
        Self {
            client: Cow::Borrowed(&CLIENT),
            url: String::from("https://overpass-api.de/api/interpreter"),
        }
    }
}

impl Overpass for OverpassServer {
    async fn evaluate(&self, query: &Query<'_>) -> Result<OverpassResult, OverpassError> {
        let mut body = String::new();
        query.fmt_oql(&mut body).map_err(|e| OverpassError::Query(e))?;

        let req = self.client.post(&self.url).body(body).build()
            .map_err(|e| OverpassError::Request(e))?;
        let res = self.client.execute(req).await
            .map_err(|e| OverpassError::Request(e))?;

        match res.bytes().await {
            Err(e) => Err(OverpassError::Request(e)),
            Ok(b) => serde_json::from_slice(&b).map_err(|e| OverpassError::Parse(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use crate::{ElementId, FilterSet, FilterType, Set};

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn server() {
        let q = OverpassServer::default().evaluate(
            &Query::from(Set::Filter(FilterSet {
                filter_type: FilterType::Node,
                id_filters: HashSet::from([3359850618]),
                ..Default::default()
            })),
        ).await;

        let ids = q.unwrap().elements.into_iter().map(|e| e.id()).collect::<HashSet<ElementId>>();
        assert_eq!(ids, HashSet::from([ElementId::Node(3359850618)]));
    }
}

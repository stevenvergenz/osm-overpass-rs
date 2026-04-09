use crate::{Overpass, OverpassError, OverpassQL, OverpassResult, Query};
use reqwest::Client;
use std::{borrow::Cow, sync::LazyLock};

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

/// Makes an HTTP request to an Overpass API server to evaluate queries.
///
/// This implements [Default] with the official API server URL and a shared static HTTP client.
/// This means you will usually not need to construct your own instance, you can use
/// `OverpassServer::default()` directly.
#[derive(Debug)]
pub struct OverpassServer {
    pub client: Cow<'static, Client>,
    pub url: String,
}

impl Default for OverpassServer {
    fn default() -> Self {
        Self {
            client: Cow::Borrowed(&CLIENT),
            #[cfg(not(test))]
            url: String::from("https://overpass-api.de/api/interpreter"),
            #[cfg(test)]
            url: String::from("http://localhost:8081/api/interpreter"),
        }
    }
}

impl Overpass for OverpassServer {
    async fn evaluate(
        &self,
        query: &Query<'_>,
    ) -> Result<OverpassResult, OverpassError> {
        let mut body = String::new();
        query
            .fmt_oql(&mut body)
            .map_err(|e| OverpassError::Query(e))?;

        let req = self
            .client
            .post(&self.url)
            .body(body)
            .build()
            .map_err(|e| OverpassError::Request(e))?;
        let res = self
            .client
            .execute(req)
            .await
            .map_err(|e| OverpassError::Request(e))?
            .error_for_status()
            .map_err(|e| OverpassError::Request(e))?;

        match res.bytes().await {
            Err(e) => Err(OverpassError::Request(e)),
            Ok(b) => {
                let res = String::from_utf8_lossy(&b).to_string();
                serde_json::from_slice(&b)
                    .map_err(|e| OverpassError::Parse(e, res))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Bbox, ElementId, SetBuilder, SetBuilderCommon};
    use std::collections::HashSet;

    use super::*;

    #[tokio::test]
    async fn server() {
        let q = OverpassServer::default()
            .evaluate(
                SetBuilder::nodes()
                    .with_id(3359850618)
                    .to_query()
                    .search_bbox(Bbox {
                        north: 47.667,
                        south: 47.553,
                        east: -122.201,
                        west: -122.461,
                    })
                    .as_ref(),
            )
            .await;

        let ids = q
            .expect("Evaluation failure")
            .elements
            .into_iter()
            .map(|e| e.id())
            .collect::<HashSet<_>>();
        assert_eq!(ids, HashSet::from([ElementId::Node(3359850618)]));
    }
}

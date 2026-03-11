//!
//! # osm_overpass
//! 
//! Easily create and execute [Overpass API](https://wiki.openstreetmap.org/wiki/Overpass_API)
//! queries of [OpenStreetMap](https://wiki.openstreetmap.org/wiki/About_OpenStreetMap). Supports
//! both declarative and builder syntax for generating OverpassQL strings, a [reqwest]-based
//! submission API, and type definitions for handling the query results.
//!
//! # Examples
//!
//! A basic query:
//!
//! ```
//! # use osm_overpass::*;
//! # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! // find all the landmarks in downtown Seattle
//! let query: Query = SetBuilder::all_nodes_or_ways()
//!     .with_tag_value("seamark:type", "landmark")
//!     .to_query()
//!     .global_bbox(Bbox { north: 47.667, west: -122.461, south: 47.553, east: -122.201 })
//!     .into();
//!
//! assert_eq!(query.to_oql(), [
//!     "[bbox:47.553,-122.461,47.667,-122.201][out:json];",
//!     r#"nw["seamark:type"="landmark"];"#,
//!     "out;",
//! ].join(""));
//!
//! // those landmarks should contain the Space Needle
//! let res: OverpassResult = OverpassServer::default().evaluate(&query).await.unwrap();
//! assert!(res.elements.iter().any(|e| matches!(e.tag("name"), Some("Space Needle"))));
//! # });
//! ```

mod osm;
pub use osm::*;

mod query;
pub use query::*;

mod overpass;
pub use overpass::*;

mod builder;
pub use builder::*;

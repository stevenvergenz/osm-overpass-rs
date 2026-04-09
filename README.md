# overpass-lib

Easily create and execute [Overpass API](https://wiki.openstreetmap.org/wiki/Overpass_API)
queries of [OpenStreetMap](https://wiki.openstreetmap.org/wiki/About_OpenStreetMap). Supports
both declarative and builder syntax for generating OverpassQL strings, a [reqwest]-based
submission API, and type definitions for handling the query results.

## Usage

Just like in OverpassQL, you specify a [Set] of [Element]s that you want, defined either as a basic [FilterSet]
or as compositions of other sets (e.g. [UnionSet]). You can either define these structs directly, or use
[SetBuilder] for more convenient syntax.

Once your set is specified, you:

1. Convert it to a [Query] or [QueryBuilder],
1. Apply any top-level settings to the query (e.g. [Query::search_bbox]),
1. Evaluate the query via an Overpass API server ([OverpassServer]),
1. Do whatever you like with the [Element]s returned!


## Examples

The easiest way to build a query is with the builder syntax:

```rust
# use std::collections::HashSet;
# use overpass_lib::*;
// Let's find all the landmarks in downtown Seattle in a standard OverpassQL string.
let oql = [
    "[bbox:47.553,-122.461,47.667,-122.201][out:json];",
    r#"nw["seamark:type"="landmark"]->.a;"#,
    ".a out;",
].join("");

let builder_query: Query = SetBuilder::nodes_or_ways()
    .with_tag_value("seamark:type", "landmark")
    .to_query()
    .search_bbox(Bbox {
        north: 47.667,
        south: 47.553,
        east: -122.201,
        west: -122.461,
    })
    .into();
assert_eq!(&oql, &builder_query.to_oql());
```

If you prefer the declarative version instead, you're free to use it:

```rust
# use std::collections::HashSet;
# use overpass_lib::*;
# let oql = [
#     "[bbox:47.553,-122.461,47.667,-122.201][out:json];",
#     r#"nw["seamark:type"="landmark"]->.a;"#,
#     ".a out;",
# ].join("");
let dec_query = Query {
    outputs: vec![
        QueryOutput {
            set: Set::from(FilterSet {
                filter_type: FilterType::NodeOrWay,
                tag_filters: HashSet::from([TagFilter::equals("seamark:type", "landmark")]),
                ..Default::default()
            }).into(),
            ..Default::default()
        },
    ],
    search_bbox: Some(Bbox {
        north: 47.667,
        south: 47.553,
        east: -122.201,
        west: -122.461,
    }),
    ..Default::default()
};
assert_eq!(&oql, &dec_query.to_oql());
```

Evaluate the query via the default Overpass API server:

```rust
# use std::collections::HashSet;
# use overpass_lib::*;
# let query: Query = SetBuilder::nodes_or_ways()
#    .with_tag_value("seamark:type", "landmark")
#    .to_query()
#    .search_bbox(Bbox { north: 47.667, south: 47.553, east: -122.201, west: -122.461 })
#    .into();
# tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {

let res = OverpassServer::default().evaluate(&query).await.unwrap();

// One of those landmarks should be the Space Needle.
assert!(
    res.elements.iter().any(|e| {
        matches!(
            e.tags().get("name").map(|s| s.as_str()),
            Some("Space Needle"),
        )
    }),
);
# });
```
## Language Support

* Settings
    * ✅ timeout
    * ✅ maxsize
    * ✅ bbox
    * ✅ date
    * ✅ diff
    * ❌ adiff
    * ✅ out count
    * ✅ out verbosity
    * ✅ out modificators
    * ❌ out bbox
    * ❌ out sort order
    * ❌ out limit
* Sets
    * ✅ union
    * ❌ difference
    * ✅ intersection
    * ❌ if-block
    * ❌ foreach
    * ❌ for
    * ❌ complete
    * ❌ retro
    * ❌ compare
    * ❌ recurse up
    * ❌ recurse up relations
    * ❌ recurse down
    * ❌ recurse down relations
    * ❌ is_in
    * ❌ timeline
    * ❌ local
    * ❌ convert
    * ❌ make
    * Filters
        * ✅ has-kv
        * ✅ bbox
        * ✅ recurse refs
        * ❌ recurse way cnt/link
        * ✅ input set
        * ✅ id
        * ❌ around
        * ❌ poly
        * ❌ newer
        * ❌ changed
        * ❌ user
        * ❌ area
        * ❌ pivot
        * ❌ if
    * ❌ evaluators

## Contributing

Issues and pull requests welcome through
[GitHub](https://github.com/stevenvergenz/osm-overpass-rs).

License: MIT

[Set]: https://docs.rs/overpass-lib/latest/overpass_lib/enum.Set.html
[Query::search_bbox]: https://docs.rs/overpass-lib/latest/overpass_lib/struct.Query.html#structfield.search_bbox

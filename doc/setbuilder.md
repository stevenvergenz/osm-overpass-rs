# SetBuilder

SetBuilder is the main entrypoint for constructing [Set]s. It exposes methods
that return several different types of builders depending on the desired type
of set: [FilterSetBuilder]s for [FilterSet]s, [UnionSetBuilder]s for [UnionSet]s,
etc.

## Filter Sets

The basic set type is the [FilterSet]. You specify properties that all elements in the set must have, and it is
populated with the matching elements during evaluation. For example:

```
# use overpass_lib::*;
let simple_set = SetBuilder::nodes().with_tag_value("public_transport", "platform");

let complex_set = SetBuilder::relations()
    .with_tag_values([("type", "route"), ("public_transport", "route")])
    .containing_nodes(simple_set);
```

Note that in the complex set above, the [containing_nodes](crate::FilterSetBuilder::containing_nodes) call
takes an owned `FilterSetBuilder`, but it could have instead taken a different builder type, an owned `Set` value,
or a borrowed `&Set`:

```
# use overpass_lib::*;

// with a borrowed set
let simple_set: Set = SetBuilder::nodes().with_tag_value("public_transport", "platform").into();
let complex_set = SetBuilder::relations()
    .with_tag_values([("type", "route"), ("public_transport", "route")])
    .containing_nodes(&simple_set);

// with an owned set
let simple_set: Set = SetBuilder::nodes().with_tag_value("public_transport", "platform").into();
let complex_set = SetBuilder::relations()
    .with_tag_values([("type", "route"), ("public_transport", "route")])
    .containing_nodes(simple_set);
```

## Union Sets

The [UnionSet]
# SetBuilder

SetBuilder is the main entrypoint for constructing [Set]s. It exposes methods
that return several different types of builders depending on the desired type
of set: [FilterSetBuilder]s for [FilterSet]s, [UnionSetBuilder]s for [UnionSet]s,
etc.

The basic set type is the [FilterSet]. 
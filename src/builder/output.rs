use crate::{
    Bbox, Query, QueryBuilder, QueryGeometry, QueryOutput, QueryVerbosity, Set,
    SortOrder,
};
use std::borrow::Cow;

/// Convenience builder for creating [QueryOutput]s.
pub struct OutputBuilder<'a>(pub Vec<QueryOutput<'a>>);

impl<'a> OutputBuilder<'a> {
    /// Create a new output builder for the given set.
    pub fn new(set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        Self(vec![QueryOutput {
            set: set.into(),
            ..Default::default()
        }])
    }

    /// Create an additional output for the given set. Subsequent settings will be applied to the
    /// new output.
    pub fn next(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.push(QueryOutput {
            set: set.into(),
            ..Default::default()
        });
        self
    }

    /// Set the verbosity of the output set's elements.
    pub fn verbosity(mut self, verbosity: QueryVerbosity) -> Self {
        if let Some(o) = self.0.last_mut() {
            o.verbosity = verbosity;
        }
        self
    }

    /// Set whether geometry should be computed for the output set's elements.
    pub fn geometry(mut self, geometry: QueryGeometry) -> Self {
        if let Some(o) = self.0.last_mut() {
            o.geo = geometry;
        }
        self
    }

    /// Restrict geometry output to these bounds.
    pub fn bbox(mut self, bbox: Bbox) -> Self {
        if let Some(o) = self.0.last_mut() {
            o.bbox = Some(bbox);
        }
        self
    }

    /// Change how returned elements are sorted.
    pub fn sort_order(mut self, sort: SortOrder) -> Self {
        if let Some(o) = self.0.last_mut() {
            o.sort = sort;
        }
        self
    }

    /// Restrict the output element count.
    pub fn limit(mut self, limit: usize) -> Self {
        if let Some(o) = self.0.last_mut() {
            o.limit = Some(limit);
        }
        self
    }

    /// Convert to a [QueryBuilder].
    pub fn to_query(self) -> QueryBuilder<'a> {
        QueryBuilder(Query {
            outputs: self.0,
            ..Default::default()
        })
    }
}

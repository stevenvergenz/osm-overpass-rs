use crate::{Bbox, Query, QueryBuilder, QueryGeometry, QueryOutput, QueryVerbosity, SortOrder};

/// Convenience builder for creating a [QueryOutput].
pub struct OutputBuilder<'a>(pub QueryOutput<'a>);

impl<'a> OutputBuilder<'a> {
    /// Set the verbosity of the output set's elements.
    pub fn verbosity(mut self, verbosity: QueryVerbosity) -> Self {
        self.0.verbosity = verbosity;
        self
    }

    /// Set whether geometry should be computed for the output set's elements.
    pub fn geometry(mut self, geometry: QueryGeometry) -> Self {
        self.0.geo = geometry;
        self
    }

    /// Restrict geometry output to these bounds.
    pub fn bbox(mut self, bbox: Bbox) -> Self {
        self.0.bbox = Some(bbox);
        self
    }

    /// Change how returned elements are sorted.
    pub fn sort_order(mut self, sort: SortOrder) -> Self {
        self.0.sort = sort;
        self
    }

    /// Restrict the output element count.
    pub fn limit(mut self, limit: usize) -> Self {
        self.0.limit = Some(limit);
        self
    }

    /// Convert to a [QueryBuilder].
    pub fn to_query(self) -> QueryBuilder<'a> {
        QueryBuilder(Query {
            outputs: vec![self.into()],
            ..Default::default()
        })
    }
}

impl<'a> Into<QueryOutput<'a>> for OutputBuilder<'a> {
    fn into(self) -> QueryOutput<'a> {
        self.0
    }
}

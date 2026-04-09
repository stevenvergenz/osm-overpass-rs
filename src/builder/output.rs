use crate::{Query, QueryBuilder, QueryGeometry, QueryOutput, QueryVerbosity};

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

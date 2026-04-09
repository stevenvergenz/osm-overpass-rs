use crate::{Query, QueryBuilder, QueryGeometry, QueryOutput, QueryVerbosity};

pub struct OutputBuilder<'a>(pub QueryOutput<'a>);

impl<'a> OutputBuilder<'a> {
    /// Set [Query::verbosity].
    pub fn verbosity(mut self, verbosity: QueryVerbosity) -> Self {
        self.0.verbosity = verbosity;
        self
    }

    pub fn geometry(mut self, geometry: QueryGeometry) -> Self {
        self.0.geo = geometry;
        self
    }

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

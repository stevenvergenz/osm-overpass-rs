use chrono::{DateTime, Utc};
use crate::{Bbox, FilterSet, FilterSetBuilder, Query, QueryVerbosity, Set, UnionSet, UnionSetBuilder};

pub trait ToQuery<'a>: Into<QueryBuilder<'a>> {
    fn to_query(self) -> QueryBuilder<'a> {
        self.into()
    }
}

pub struct QueryBuilder<'a>(
    Query<'a>,
);

impl<'a> Into<Query<'a>> for QueryBuilder<'a> {
    fn into(self) -> Query<'a> {
        self.0
    }
}

impl<'a> AsRef<Query<'a>> for QueryBuilder<'a> {
    fn as_ref(&self) -> &Query<'a> {
        &self.0
    }
}

impl<'a> QueryBuilder<'a> {
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.0.timeout_s = Some(timeout);
        self
    }

    pub fn max_size(mut self, max_size: u32) -> Self {
        self.0.max_size = Some(max_size);
        self
    }

    pub fn global_bbox(mut self, bbox: impl Into<Bbox>) -> Self {
        self.0.search_bbox = Some(bbox.into());
        self
    }

    pub fn as_of_date(mut self, date: impl Into<DateTime<Utc>>) -> Self {
        self.0.as_of_date = Some(date.into());
        self
    }

    pub fn diff_since(mut self, start: impl Into<DateTime<Utc>>) -> Self {
        self.0.diff = Some((start.into(), None));
        self
    }

    pub fn diff_range(mut self, start: impl Into<DateTime<Utc>>, end: impl Into<DateTime<Utc>>) -> Self {
        self.0.diff = Some((start.into(), Some(end.into())));
        self
    }

    pub fn data_type(mut self, r#type: QueryVerbosity) -> Self {
        self.0.verbosity = r#type;
        self
    }
}

impl<'a> Into<QueryBuilder<'a>> for FilterSetBuilder<'a> {
    fn into(self) -> QueryBuilder<'a> {
        let set: FilterSet = self.0.into();
        QueryBuilder(Query::from(Set::from(set)))
    }
}
impl<'a> ToQuery<'a> for FilterSetBuilder<'a> {}

impl<'a> Into<QueryBuilder<'a>> for UnionSetBuilder<'a> {
    fn into(self) -> QueryBuilder<'a> {
        let set: UnionSet = self.0.into();
        QueryBuilder(Query::from(Set::from(set)))
    }
}
impl<'a> ToQuery<'a> for UnionSetBuilder<'a> {}

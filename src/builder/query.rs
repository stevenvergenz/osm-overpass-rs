use std::time::Duration;
use chrono::{DateTime, Utc};
use crate::{Bbox, FilterSet, FilterSetBuilder, Query, Set, UnionSet, UnionSetBuilder};

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

impl<'a> QueryBuilder<'a> {
    pub fn with_timeout(mut self, timeout: impl Into<Duration>) -> Self {
        self.0.timeout = Some(timeout.into());
        self
    }

    pub fn with_max_size(mut self, max_size: u32) -> Self {
        self.0.max_size = Some(max_size);
        self
    }

    pub fn with_global_bbox(mut self, bbox: impl Into<Bbox>) -> Self {
        self.0.global_bbox = Some(bbox.into());
        self
    }

    pub fn as_of_date(mut self, date: impl Into<DateTime<Utc>>) -> Self {
        self.0.as_of_date = Some(date.into());
        self
    }

    pub fn with_diff_range(mut self, start: impl Into<DateTime<Utc>>, end: impl Into<Option<DateTime<Utc>>>) -> Self {
        self.0.diff = Some((start.into(), end.into()));
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

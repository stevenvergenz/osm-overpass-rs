use chrono::{DateTime, Utc};
use crate::{Bbox, FilterSet, FilterSetBuilder, Query, QueryVerbosity, Set, UnionSet, UnionSetBuilder};
#[cfg(doc)]
use crate::SetBuilder;

/// Trait to convert [SetBuilder]s into [QueryBuilder]s unambiguously in addition to
/// [`Into<QueryBuilder>`].
pub trait ToQuery<'a>: Into<QueryBuilder<'a>> {
    /// Convert this type into a [QueryBuilder].
    fn to_query(self) -> QueryBuilder<'a> {
        self.into()
    }
}

/// A convenient builder API for [Query].
pub struct QueryBuilder<'a>(
    /// The query being modified.
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
    /// Set [Query::timeout_s].
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.0.timeout_s = Some(timeout);
        self
    }

    /// Set [Query::max_size].
    pub fn max_size(mut self, max_size: u32) -> Self {
        self.0.max_size = Some(max_size);
        self
    }

    /// Set [Query::search_bbox].
    pub fn search_bbox(mut self, bbox: impl Into<Bbox>) -> Self {
        self.0.search_bbox = Some(bbox.into());
        self
    }

    /// Set [Query::as_of_date].
    pub fn as_of_date(mut self, date: impl Into<DateTime<Utc>>) -> Self {
        self.0.as_of_date = Some(date.into());
        self
    }

    /// Set [Query::diff] with no end date.
    pub fn diff_since(mut self, start: impl Into<DateTime<Utc>>) -> Self {
        self.0.diff = Some((start.into(), None));
        self
    }

    /// Set [Query::diff] with an end date.
    pub fn diff_range(mut self, start: impl Into<DateTime<Utc>>, end: impl Into<DateTime<Utc>>) -> Self {
        self.0.diff = Some((start.into(), Some(end.into())));
        self
    }

    /// Set [Query::verbosity].
    pub fn verbosity(mut self, verbosity: QueryVerbosity) -> Self {
        self.0.verbosity = verbosity;
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

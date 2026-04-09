#[cfg(doc)]
use crate::SetBuilder;
use crate::{Bbox, FilterSetBuilder, Query, QueryOutput, UnionSetBuilder};
use chrono::{DateTime, Utc};

/// A convenient builder API for [Query].
pub struct QueryBuilder<'a>(
    /// The query being modified.
    pub Query<'a>,
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
    pub fn diff_range(
        mut self,
        start: impl Into<DateTime<Utc>>,
        end: impl Into<DateTime<Utc>>,
    ) -> Self {
        self.0.diff = Some((start.into(), Some(end.into())));
        self
    }
}

impl<'a> From<FilterSetBuilder<'a>> for QueryBuilder<'a> {
    fn from(value: FilterSetBuilder<'a>) -> Self {
        Self(Query {
            outputs: vec![QueryOutput {
                set: value.into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    }
}

impl<'a> From<UnionSetBuilder<'a>> for QueryBuilder<'a> {
    fn from(value: UnionSetBuilder<'a>) -> Self {
        Self(Query {
            outputs: vec![QueryOutput {
                set: value.into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    }
}

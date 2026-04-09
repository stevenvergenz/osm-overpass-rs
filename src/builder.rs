mod filter;
pub use filter::*;

mod union;
pub use union::*;

mod query;
pub use query::*;

mod output;
pub use output::*;

use crate::{Query, QueryOutput, Set};
use enum_dispatch::enum_dispatch;
use std::borrow::Cow;

/// Trait to maintain consistency between builder types.
#[enum_dispatch]
pub trait SetBuilderCommon<'a>:
    Into<Set<'a>> + Into<Cow<'a, Set<'a>>> + IntoIterator<Item = Self>
{
    /// Create a new set with all elements from both this and another set.
    fn union_with(
        self,
        other: impl Into<Cow<'a, Set<'a>>>,
    ) -> UnionSetBuilder<'a>;

    /// Start configuring output options for this set.
    fn to_output(self) -> OutputBuilder<'a> {
        OutputBuilder(QueryOutput {
            set: self.into(),
            ..Default::default()
        })
    }

    /// Start configuring query options for this set.
    fn to_query(self) -> QueryBuilder<'a> {
        QueryBuilder(Query {
            outputs: vec![self.to_output().into()],
            ..Default::default()
        })
    }
}

/// An enum of all the types of builders that produce sets.
#[doc = include_str!("../doc/setbuilder.md")]
#[derive(Debug, Clone)]
#[enum_dispatch(SetBuilderCommon)]
pub enum SetBuilder<'a> {
    /// Builds a filter set.
    Filter(FilterSetBuilder<'a>),
    /// Builds a union set.
    Union(UnionSetBuilder<'a>),
}

impl<'a> Into<Set<'a>> for SetBuilder<'a> {
    fn into(self) -> Set<'a> {
        match self {
            Self::Filter(f) => f.into(),
            Self::Union(u) => u.into(),
        }
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for SetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Owned(self.into())
    }
}

impl<'a> IntoIterator for SetBuilder<'a> {
    type Item = SetBuilder<'a>;
    type IntoIter = std::array::IntoIter<SetBuilder<'a>, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

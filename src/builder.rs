mod filter;
pub use filter::*;

mod union;
pub use union::*;

mod query;
pub use query::*;

mod output;
pub use output::*;

mod common;
pub use common::*;

use crate::Set;
use std::borrow::Cow;

/// An enum of all the types of builders that produce sets.
#[doc = include_str!("../doc/setbuilder.md")]
#[derive(Debug, Clone)]
pub enum SetBuilder<'a> {
    /// Builds a filter set.
    Filter(FilterSetBuilder<'a>),
    /// Builds a union set.
    Union(UnionSetBuilder<'a>),
}

impl<'a> SetBuilderCommon<'a> for SetBuilder<'a> {
    type Inner = Set<'a>;
    fn inner(&mut self) -> &mut Self::Inner {
        match self {
            Self::Filter(f) => &mut f.0,
            Self::Union(u) => &mut u.0,
        }
    }
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

impl<'a> Into<Cow<'a, Set<'a>>> for &'a SetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Borrowed(self.as_ref())
    }
}

impl<'a> IntoIterator for SetBuilder<'a> {
    type Item = SetBuilder<'a>;
    type IntoIter = std::array::IntoIter<SetBuilder<'a>, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

impl<'a> AsRef<Set<'a>> for SetBuilder<'a> {
    fn as_ref(&self) -> &Set<'a> {
        match self {
            Self::Filter(s) => s.as_ref(),
            Self::Union(s) => s.as_ref(),
        }
    }
}

impl<'a> AsMut<Set<'a>> for SetBuilder<'a> {
    fn as_mut(&mut self) -> &mut Set<'a> {
        match self {
            Self::Filter(s) => s.as_mut(),
            Self::Union(u) => u.as_mut(),
        }
    }
}

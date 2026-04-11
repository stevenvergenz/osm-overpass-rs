use crate::{Set, SetBuilder, SetBuilderCommon, UnionSet};
use std::borrow::Cow;

/// A convenient builder API for [UnionSet].
#[derive(Debug, Clone)]
pub struct UnionSetBuilder<'a>(
    /// The set being modified.
    pub UnionSet<'a>,
);

impl SetBuilder<'_> {
    /// Collect the provided sets into a new [UnionSet]
    pub fn union<'a, T>(
        sets: impl IntoIterator<Item = T>,
    ) -> UnionSetBuilder<'a>
    where
        T: Into<Cow<'a, Set<'a>>>,
    {
        UnionSetBuilder(sets.into_iter().collect())
    }
}

impl<'a> SetBuilderCommon<'a> for UnionSetBuilder<'a> {
    fn union_with(mut self, other: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.0.insert(other.into());
        self
    }
}

impl<'a> Into<Set<'a>> for UnionSetBuilder<'a> {
    fn into(self) -> Set<'a> {
        self.0.into()
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for UnionSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Owned(self.into())
    }
}

impl<'a> IntoIterator for UnionSetBuilder<'a> {
    type Item = UnionSetBuilder<'a>;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

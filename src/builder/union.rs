use crate::{FilterSetBuilder, Set, SetBuilder, UnionSet, builder::Builder};
use std::{borrow::Cow, collections::HashSet};

/// Trait to daisy-chain [SetBuilder]s together into [UnionSetBuilder]s.
pub trait UnionWith<'a> {
    /// Combine this [Set] together with another into a [UnionSet].
    fn union_with(self, other: impl Into<Cow<'a, Set<'a>>>) -> UnionSetBuilder<'a>;
}

/// A convenient builder API for [UnionSet].
pub struct UnionSetBuilder<'a>(
    /// The set being modified.
    pub UnionSet<'a>,
);

impl<'a> Builder<'a> for UnionSetBuilder<'a> {}

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

impl<'a> UnionWith<'a> for FilterSetBuilder<'a> {
    fn union_with(self, other: impl Into<Cow<'a, Set<'a>>>) -> UnionSetBuilder<'a> {
        UnionSetBuilder(UnionSet(HashSet::from([self.into(), other.into()])))
    }
}

impl<'a> UnionWith<'a> for UnionSetBuilder<'a> {
    fn union_with(mut self, other: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.0.insert(other.into());
        self
    }
}

impl SetBuilder {
    /// Collect the provided sets into a new [UnionSet]
    pub fn union<'a, T>(sets: impl IntoIterator<Item = T>) -> UnionSetBuilder<'a>
    where
        T: Into<Cow<'a, Set<'a>>>,
    {
        UnionSetBuilder(sets.into_iter().collect())
    }
}

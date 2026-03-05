use std::{borrow::Cow, collections::HashSet};
use crate::{FilterSetBuilder, Set, SetBuilder, UnionSet, builder::Builder};

pub trait UnionWith<'a> {
    fn union_with(self, other: impl Into<Cow<'a, Set<'a>>>) -> UnionSetBuilder<'a>;
}

pub struct UnionSetBuilder<'a>(
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
    pub fn union<'a, T>(sets: impl IntoIterator<Item=T>) -> UnionSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        UnionSetBuilder(sets.into_iter().collect())
    }
}


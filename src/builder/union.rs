use std::{borrow::Cow, collections::HashSet};
use crate::{SetBuilder, FilterSetBuilder, Set, UnionSet};

pub struct UnionSetBuilder<'a>(
    UnionSet<'a>,
);

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

impl SetBuilder {
    pub fn union<'a, T>(sets: impl IntoIterator<Item=T>) -> UnionSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        UnionSetBuilder(sets.into_iter().collect())
    }
}

impl<'a> UnionSetBuilder<'a> {
    pub fn union_with(mut self, other: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.0.insert(other.into());
        self
    }
}

impl<'a> FilterSetBuilder<'a> {
    pub fn union_with(self, other: impl Into<Cow<'a, Set<'a>>>) -> UnionSetBuilder<'a> {
        UnionSetBuilder(UnionSet(HashSet::from([self.into(), other.into()])))
    }
}

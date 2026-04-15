use crate::{Set, SetBuilder, SetBuilderCommon, UnionSet};
use std::borrow::Cow;

/// A convenient builder API for [UnionSet].
#[derive(Debug, Clone)]
pub struct UnionSetBuilder<'a>(
    /// The set being modified.
    pub Set<'a>,
);

/// Methods to build new [UnionSet]s.
impl SetBuilder<'_> {
    /// Collect the provided sets into a new [UnionSet]
    pub fn union<'a, T>(
        sets: impl IntoIterator<Item = T>,
    ) -> UnionSetBuilder<'a>
    where
        T: Into<Cow<'a, Set<'a>>>,
    {
        UnionSetBuilder(Set::Union(sets.into_iter().collect()))
    }
}

impl<'a> SetBuilderCommon<'a> for UnionSetBuilder<'a> {
    type Inner = UnionSet<'a>;

    fn inner(&mut self) -> &mut Self::Inner {
        match &mut self.0 {
            Set::Union(s) => s,
            _ => panic!("bad variant"),
        }
    }

    fn union_with(mut self, other: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.inner().0.insert(other.into());
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
        self.0.into()
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for &'a UnionSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        self.as_ref().into()
    }
}

impl<'a> IntoIterator for UnionSetBuilder<'a> {
    type Item = Self;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

impl<'a> AsRef<Set<'a>> for UnionSetBuilder<'a> {
    fn as_ref(&self) -> &Set<'a> {
        &self.0
    }
}

impl<'a> AsMut<Set<'a>> for UnionSetBuilder<'a> {
    fn as_mut(&mut self) -> &mut Set<'a> {
        &mut self.0
    }
}

impl<'a, A> FromIterator<A> for UnionSetBuilder<'a>
where
    A: Into<Cow<'a, Set<'a>>>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(Set::Union(UnionSet::from_iter(iter)))
    }
}

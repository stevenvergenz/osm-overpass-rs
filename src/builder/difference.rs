use std::borrow::Cow;

use crate::{DifferenceSet, Set, SetBuilder, SetBuilderCommon};

/// Configure a [DifferenceSet].
#[derive(Debug, Clone)]
pub struct DifferenceSetBuilder<'a>(pub Set<'a>);

/// Methods to create new [DifferenceSet]s.
impl<'a> SetBuilder<'a> {
    /// Remove one set's elements from another set.
    pub fn difference(
        input: impl Into<Cow<'a, Set<'a>>>,
        exclude: impl Into<Cow<'a, Set<'a>>>,
    ) -> DifferenceSetBuilder<'a> {
        DifferenceSetBuilder::new(input, exclude)
    }
}

impl<'a> DifferenceSetBuilder<'a> {
    /// Create a new difference set builder.
    pub fn new(
        input: impl Into<Cow<'a, Set<'a>>>,
        exclude: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        Self(DifferenceSet::new(input, exclude).into())
    }
}

impl<'a> SetBuilderCommon<'a> for DifferenceSetBuilder<'a> {
    type Inner = DifferenceSet<'a>;
    fn inner(&mut self) -> &mut Self::Inner {
        match &mut self.0 {
            Set::Difference(d) => d,
            _ => panic!("bad variant"),
        }
    }
}

impl<'a> Into<Set<'a>> for DifferenceSetBuilder<'a> {
    fn into(self) -> Set<'a> {
        self.0
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for DifferenceSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        self.0.into()
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for &'a DifferenceSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Borrowed(self.as_ref())
    }
}

impl<'a> IntoIterator for DifferenceSetBuilder<'a> {
    type Item = Self;
    type IntoIter = std::array::IntoIter<Self, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

impl<'a> AsRef<Set<'a>> for DifferenceSetBuilder<'a> {
    fn as_ref(&self) -> &Set<'a> {
        &self.0
    }
}

impl<'a> AsMut<Set<'a>> for DifferenceSetBuilder<'a> {
    fn as_mut(&mut self) -> &mut Set<'a> {
        &mut self.0
    }
}

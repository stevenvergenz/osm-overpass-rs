use std::borrow::Cow;

use crate::{RecurseSet, Set, SetBuilderCommon};

/// A builder struct for a [RecurseSet].
#[derive(Debug, Clone)]
pub struct RecurseSetBuilder<'a>(
    pub Set<'a>,
);

impl<'a> SetBuilderCommon<'a> for RecurseSetBuilder<'a> {
    type Inner = RecurseSet<'a>;
    fn inner(&mut self) -> &mut Self::Inner {
        match &mut self.0 {
            Set::Recurse(r) => r,
            _ => panic!("bad variant"),
        }
    }
}

impl<'a> Into<Set<'a>> for RecurseSetBuilder<'a> {
    fn into(self) -> Set<'a> {
        self.0
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for RecurseSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        self.0.into()
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for &'a RecurseSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        self.as_ref().into()
    }
}

impl<'a> IntoIterator for RecurseSetBuilder<'a> {
    type Item = Self;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

impl<'a> AsRef<Set<'a>> for RecurseSetBuilder<'a> {
    fn as_ref(&self) -> &Set<'a> {
        &self.0
    }
}

impl<'a> AsMut<Set<'a>> for RecurseSetBuilder<'a> {
    fn as_mut(&mut self) -> &mut Set<'a> {
        &mut self.0
    }
}
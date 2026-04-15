#[cfg(doc)]
use crate::Element;
use crate::{
    DifferenceSet, FilterSet, Namer, OverpassQLError, OverpassQLNamed, RecurseSet, UnionSet
};
use std::{
    borrow::Cow,
    collections::HashSet,
    fmt::Write,
    hash::{Hash, Hasher},
};

/// An abstract collection of [Element]s selected from the full database based on given criteria.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Sets)
#[derive(Debug, Clone)]
pub enum Set<'a> {
    /// The standard query statement. See [FilterSet].
    Filter(FilterSet<'a>),
    /// The union block statement. See [UnionSet].
    Union(UnionSet<'a>),
    /// The difference block statement. See [DifferenceSet].
    Difference(DifferenceSet<'a>),
    /// A set whose elements are determined by their relationship to elements in an input set. See [RecurseSet].
    Recurse(RecurseSet<'a>),
    /// A string of raw OverpassQL that generates a set. May cause the query to not compile!
    Raw(String),
}

impl Default for Set<'_> {
    fn default() -> Self {
        Self::Filter(FilterSet::default())
    }
}

impl<'a> OverpassQLNamed<'a> for Set<'a> {
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c,
    {
        match self {
            Self::Filter(filter) => filter.fmt_oql_named(f, namer)?,
            Self::Union(union) => union.fmt_oql_named(f, namer)?,
            Self::Difference(s) => s.fmt_oql_named(f, namer)?,
            Self::Recurse(r) => r.fmt_oql_named(f, namer)?,
            Self::Raw(raw) => write!(f, "{raw}")?,
        };

        write!(f, "->.{}", namer.get(self))?;

        Ok(())
    }
}

impl<'a> Set<'a> {
    /// Returns an iterator of sets that must be defined before this one.
    pub fn dependencies(&self) -> impl ExactSizeIterator<Item = &Set<'a>> {
        match self {
            Self::Filter(s) => s.dependencies(),
            Self::Union(s) => s.dependencies(),
            Self::Difference(s) => s.dependencies(),
            Self::Recurse(r) => r.dependencies(),
            Self::Raw(_) => HashSet::new().into_iter(),
        }
    }
}

impl PartialEq for Set<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Set<'_> {}

impl Hash for Set<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for Set<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Owned(self)
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for &'a Set<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Borrowed(self)
    }
}

impl<'a> From<FilterSet<'a>> for Set<'a> {
    fn from(value: FilterSet<'a>) -> Self {
        Self::Filter(value)
    }
}

impl<'a> From<UnionSet<'a>> for Set<'a> {
    fn from(value: UnionSet<'a>) -> Self {
        Self::Union(value)
    }
}

impl<'a> From<DifferenceSet<'a>> for Set<'a> {
    fn from(value: DifferenceSet<'a>) -> Self {
        Self::Difference(value)
    }
}

impl<'a> From<RecurseSet<'a>> for Set<'a> {
    fn from(value: RecurseSet<'a>) -> Self {
        Self::Recurse(value)
    }
}


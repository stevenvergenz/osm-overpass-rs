use std::{
    borrow::Cow,
    fmt::Write,
    hash::{Hash, Hasher},
};
use crate::{
    FilterSet, Namer, OverpassQLError, OverpassQLNamed, UnionSet,
};

#[derive(Debug, Clone)]
pub enum Set<'a> {
    Filter(FilterSet<'a>),
    Union(UnionSet<'a>),
}

impl Default for Set<'_> {
    fn default() -> Self {
        Self::Filter(FilterSet::default())
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

impl<'a> OverpassQLNamed<'a> for Set<'a> {
    fn fmt_oql_named<'b, 'c>(&'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where 'b: 'c {
        match self {
            Self::Filter(filter) => filter.fmt_oql_named(f, namer),
            Self::Union(union) => union.fmt_oql_named(f, namer),
        }?;

        if let Some(name) = namer.get_or_assign(self) {
            write!(f, "->.{name}")?;
        }

        Ok(())
    }
}

impl<'a> Set<'a> {
    pub fn dependencies(&self) -> impl ExactSizeIterator<Item=&Set<'a>> {
        match self {
            Self::Filter(f) => f.dependencies(),
            Self::Union(u) => u.dependencies(),
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


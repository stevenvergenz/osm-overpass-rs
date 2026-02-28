use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FResult, Write},
    hash::{Hash, Hasher},
};
use crate::{
    FilterSet, Namer, OverpassQLError, OverpassQLNamed, OverpassQLUnnamed,
};

#[derive(Debug, Clone)]
pub enum Set<'a> {
    Filter(FilterSet<'a>),
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

impl<'a> OverpassQLNamed<'a> for Set<'a> {
    fn fmt_oql_named<'b, 'c>(&'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where 'b: 'c {
        match self {
            Self::Filter(filter) => filter.fmt_oql_named(f, namer),
        }?;

        if let Some(name) = namer.get_or_assign(self) {
            write!(f, "->.{name}").map_err(OverpassQLError::from)?;
        }

        Ok(())
    }
}

impl OverpassQLUnnamed for Set<'_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        let mut namer = Namer::new(self);
        self.fmt_oql_named(f, &mut namer)
    }
}

impl Display for Set<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
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


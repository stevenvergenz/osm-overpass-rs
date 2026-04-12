use std::{borrow::Cow, collections::HashSet, fmt::Write};

use crate::{Namer, OverpassQLError, OverpassQLNamed, Set};

/// The contents of one set without the elements found in the other set.
#[derive(Debug, Clone)]
pub struct DifferenceSet<'a> {
    /// The set that should be included.
    pub input: Box<Cow<'a, Set<'a>>>,
    /// The set that should be excluded.
    pub exclude: Box<Cow<'a, Set<'a>>>,
}

impl<'a> OverpassQLNamed<'a> for DifferenceSet<'a> {
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c,
    {
        write!(f, "(.{};-", namer.get(&self.input))?;
        write!(f, ".{};)", namer.get(&self.exclude))?;
        Ok(())
    }
}

impl<'a> DifferenceSet<'a> {
    /// The sets that must be defined before this set.
    pub fn dependencies(
        &self,
    ) -> std::collections::hash_set::IntoIter<&Set<'a>> {
        HashSet::from([
            self.input.as_ref().as_ref(),
            self.exclude.as_ref().as_ref(),
        ])
        .into_iter()
    }
}

impl<'a> TryFrom<Set<'a>> for DifferenceSet<'a> {
    type Error = &'static str;
    fn try_from(value: Set<'a>) -> Result<Self, Self::Error> {
        match value {
            Set::Difference(d) => Ok(d),
            _ => Err("bad variant"),
        }
    }
}

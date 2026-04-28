use std::{borrow::Cow, collections::HashSet, fmt::Write};

use crate::{Namer, OverpassQLError, OverpassQLNamed, Set};

/// Produces elements referencing or referenced by the elements in the input set.
#[derive(Debug, Clone)]
pub enum RecurseSet<'a> {
    /// Returns elements that are referenced by an input element.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_down_.28.3E.29).
    Down(Box<Cow<'a, Set<'a>>>),
    /// Returns elements that are referenced by an input element, and members of any found relations.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_down_relations_.28.3E.3E.29).
    DownRelations(Box<Cow<'a, Set<'a>>>),
    /// Returns elements that reference an input element.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_up_.28.3C.29).
    Up(Box<Cow<'a, Set<'a>>>),
    /// Returns elements that reference an input element, and relations that reference any found element.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_up_relations_.28.3C.3C.29).
    UpRelations(Box<Cow<'a, Set<'a>>>),
}

impl<'a> OverpassQLNamed<'a> for RecurseSet<'a> {
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c,
    {
        match self {
            Self::Down(s) => write!(f, ".{}>", namer.get(&s)),
            Self::DownRelations(s) => write!(f, ".{}>>", namer.get(&s)),
            Self::Up(s) => write!(f, ".{}<", namer.get(&s)),
            Self::UpRelations(s) => write!(f, ".{}<<", namer.get(&s)),
        }?;
        Ok(())
    }
}

impl<'a> RecurseSet<'a> {
    /// Create a [Self::Down] variant.
    pub fn down(set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        Self::Down(Box::new(set.into()))
    }

    /// Create a [Self::DownRelations] variant.
    pub fn down_relations(set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        Self::DownRelations(Box::new(set.into()))
    }

    /// Create a [Self::Up] variant.
    pub fn up(set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        Self::Up(Box::new(set.into()))
    }

    /// Create a [Self::UpRelations] variant.
    pub fn up_relations(set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        Self::UpRelations(Box::new(set.into()))
    }

    /// The input set to the recurse operation.
    pub fn input(&self) -> &Set<'a> {
        match self {
            Self::Down(s) => &s,
            Self::DownRelations(s) => &s,
            Self::Up(s) => &s,
            Self::UpRelations(s) => &s,
        }
    }

    /// The sets that must be defined before this set.
    pub fn dependencies(
        &self,
    ) -> std::collections::hash_set::IntoIter<&Set<'a>> {
        HashSet::from([self.input()]).into_iter()
    }
}

impl<'a> TryFrom<Set<'a>> for RecurseSet<'a> {
    type Error = &'static str;
    fn try_from(value: Set<'a>) -> Result<Self, Self::Error> {
        match value {
            Set::Recurse(r) => Ok(r),
            _ => Err("bad variant"),
        }
    }
}

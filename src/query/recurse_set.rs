use std::{borrow::Cow, collections::HashSet, fmt::Write};

use crate::{Namer, OverpassQLError, OverpassQLNamed, Set};

/// Produces elements referencing or referenced by the elements in the input set.
/// 
/// # Generic constants
/// 
/// * `UP` - If `true`, produces the set of elements that reference an element in the input set
///     ([wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_up_.28.3C.29)).
///     If `false`, produces the set of elements that are referenced by an element in the input set
///     ([wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_down_.28.3E.29)).
/// * `REL` - If `true`, will continue walking up/down relation members until no new relations are found.
#[derive(Debug, Clone)]
pub struct RecurseSet<'a, const UP: bool, const REL: bool>(
    pub Box<Cow<'a, Set<'a>>>,
);

impl<'a, const UP: bool, const REL: bool> OverpassQLNamed<'a> for RecurseSet<'a, UP, REL> {
    fn fmt_oql_named<'b, 'c>(
            &'b self,
            f: &mut impl Write,
            namer: &mut Namer<'a, 'c>,
        ) -> Result<(), OverpassQLError>
        where
            'b: 'c {
            match (UP, REL) {
                (false, false) => write!(f, ".{}>", namer.get(&self.0)),
                (false, true) => write!(f, ".{}>>", namer.get(&self.0)),
                (true, false) => write!(f, ".{}<", namer.get(&self.0)),
                (true, true) => write!(f, ".{}<<", namer.get(&self.0)),
            }?;
            Ok(())
    }
}

impl<'a, const UP: bool, const REL: bool> RecurseSet<'a, UP, REL> {
    pub fn new(set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        Self(Box::new(set.into()))
    }

    pub fn dependencies(&self) -> std::collections::hash_set::IntoIter<&Set<'a>> {
        HashSet::from([self.0.as_ref().as_ref()]).into_iter()
    }
}

impl<'a> TryFrom<Set<'a>> for RecurseSet<'a, false, false> {
    type Error = &'static str;
    fn try_from(value: Set<'a>) -> Result<Self, Self::Error> {
        match value {
            Set::RecurseDown(r) => Ok(r),
            _ => Err("bad variant"),
        }
    }
}

impl<'a> TryFrom<Set<'a>> for RecurseSet<'a, false, true> {
    type Error = &'static str;
    fn try_from(value: Set<'a>) -> Result<Self, Self::Error> {
        match value {
            Set::RecurseDownRelations(r) => Ok(r),
            _ => Err("bad variant"),
        }
    }
}

impl<'a> TryFrom<Set<'a>> for RecurseSet<'a, true, false> {
    type Error = &'static str;
    fn try_from(value: Set<'a>) -> Result<Self, Self::Error> {
        match value {
            Set::RecurseUp(r) => Ok(r),
            _ => Err("bad variant"),
        }
    }
}

impl<'a> TryFrom<Set<'a>> for RecurseSet<'a, true, true> {
    type Error = &'static str;
    fn try_from(value: Set<'a>) -> Result<Self, Self::Error> {
        match value {
            Set::RecurseUpRelations(r) => Ok(r),
            _ => Err("bad variant"),
        }
    }
}

use crate::{Namer, OverpassQLError, OverpassQLNamed, Set};
use std::{
    borrow::Cow,
    collections::{HashSet, hash_set::IntoIter},
    fmt::Write,
};

/// A [Set] that is composed of all elements found in any member set.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnionSet<'a>(
    /// The collection of sets whose contents make up this set.
    pub HashSet<Cow<'a, Set<'a>>>,
);

impl<'a> OverpassQLNamed<'a> for UnionSet<'a> {
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c,
    {
        write!(f, "(")?;
        for i in &self.0 {
            write!(f, ".{};", namer.get_or_assign(i))?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<'a> UnionSet<'a> {
    /// An iterator of the sets that must be defined before this set.
    pub fn dependencies(&self) -> IntoIter<&Set<'a>> {
        self.0
            .iter()
            .map(|c| c.as_ref())
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<'a, A> FromIterator<A> for UnionSet<'a>
where
    A: Into<Cow<'a, Set<'a>>>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(iter.into_iter().map(|i| i.into()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::FilterSet;

    #[test]
    fn union() {
        let set = Set::Union(UnionSet(HashSet::from([
            Cow::Owned(Set::from(FilterSet::default())),
            Cow::Owned(Set::from(FilterSet::default())),
        ])));

        let mut output = String::new();
        set.fmt_oql_named(&mut output, &mut Namer::default())
            .unwrap();
        assert_eq!(output, "(.a;.b;)->.c");
    }
}

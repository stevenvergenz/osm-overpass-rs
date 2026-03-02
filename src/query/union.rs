use std::{
    borrow::Cow,
    collections::{HashSet, hash_set::IntoIter},
    fmt::Write,
};
use crate::{Set, Namer, OverpassQLNamed, OverpassQLError};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnionSet<'a>(
    pub HashSet<Cow<'a, Set<'a>>>,
);

impl<'a> OverpassQLNamed<'a> for UnionSet<'a> {
    fn fmt_oql_named<'b, 'c>(&'b self, f: &mut impl Write, namer: &mut Namer<'a, 'c>)
    -> Result<(), OverpassQLError>
    where 'b: 'c {
        write!(f, "(")?;
        for i in &self.0 {
            if let Some(n) = namer.get_or_assign(i) {
                write!(f, ".{n};")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<'a> UnionSet<'a> {
    pub fn dependencies(&self) -> IntoIter<&Set<'a>> {
        self.0.iter().map(|c| c.as_ref())
            .collect::<HashSet<_>>().into_iter()
    }
}

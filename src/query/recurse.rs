use std::{
    borrow::Cow,
    fmt::Write,
};

use crate::{OverpassQLNamed, OverpassQLError, Namer, Set, SaniStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecurseFilter<'a> {
    WithinWays { input: Cow<'a, Set<'a>> },
    WithinRelations { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
    ContainingNodes { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
    ContainingWays { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
    ContainingRelations { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
}

impl<'a> RecurseFilter<'a> {
    pub fn input(&self) -> &Set<'a> {
        match self {
            Self::WithinWays { input } => input,
            Self::WithinRelations { input, .. } => input,
            Self::ContainingNodes { input, .. } => input,
            Self::ContainingWays { input, .. } => input,
            Self::ContainingRelations { input, .. } => input,
        }
    }
}

impl<'a> OverpassQLNamed<'a> for RecurseFilter<'a> {
    fn fmt_oql_named<'b, 'c>(&'b self, f: &mut impl Write, namer: &mut Namer<'a, 'c>)
    -> Result<(), OverpassQLError>
    where 'b: 'c {
        let (code, input, role) = match self {
            Self::WithinWays { input } => ("w", input, &None),
            Self::WithinRelations { input, role } => ("r", input, role),
            Self::ContainingNodes { input, role } => ("bn", input, role),
            Self::ContainingWays { input, role } => ("bw", input, role),
            Self::ContainingRelations { input, role } => ("br", input, role),
        };
        match (namer.get_or_assign(input), role) {
            (Some(n), Some(r)) => write!(f, "({code}.{n}:{r}")?,
            (Some(n), None) => write!(f, "({code}.{n})")?,
            (None, Some(r)) => write!(f, "({code}:{r}")?,
            (None, None) => write!(f, "({code})")?,
        };
        Ok(())
    }
}

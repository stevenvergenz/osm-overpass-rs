use std::{
    borrow::Cow,
    fmt::Write,
};

use crate::{OverpassQLNamed, OverpassQLError, Namer, Set, SaniStr};
#[cfg(doc)]
use crate::{Node, Way, Relation};

/// A filter set criterion based on the element's relationship to other elements.
/// 
/// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_.28n.2C_w.2C_r.2C_bn.2C_bw.2C_br.29)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecurseFilter<'a> {
    /// Select [Node]s that appear within a [Way] in the provided input [Set].
    WithinWays { input: Cow<'a, Set<'a>> },
    /// Select [Node]s/[Way]s that are members of a [Relation] in the provided input [Set]. If a role is specified,
    /// the element must also have that role in said relations.
    WithinRelations { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
    /// Select [Way]s/[Relation]s that have a member [Node] in the provided input [Set]. If a role is specified,
    /// the node must also have that role in said relations.
    ContainingNodes { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
    /// Select [Relation]s that have a member [Way] in the provided input [Set]. If a role is specified,
    /// the way must also have that role in said relations.
    ContainingWays { input: Cow<'a, Set<'a>>, role: Option<SaniStr<'a>> },
    /// Select [Relation]s that have a member [Relation] in the provided input [Set]. If a role is specified,
    /// the relation must also have that role in said relations.
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

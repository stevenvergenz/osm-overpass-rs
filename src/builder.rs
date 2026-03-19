mod filter;
pub use filter::*;

mod union;
pub use union::*;

mod query;
pub use query::*;

use crate::Set;
use std::borrow::Cow;

/// Internal trait to maintain consistency between builder types
#[allow(unused)]
pub(crate) trait Builder<'a>:
    Into<Set<'a>>
    + Into<Cow<'a, Set<'a>>>
    + IntoIterator<Item = Self>
    + UnionWith<'a>
    + ToQuery<'a>
{
}

#[doc = include_str!("../doc/setbuilder.md")]
pub struct SetBuilder;

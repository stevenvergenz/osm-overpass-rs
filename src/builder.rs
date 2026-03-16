mod filter;
pub use filter::*;

mod union;
pub use union::*;

mod query;
pub use query::*;

use std::borrow::Cow;
use crate::Set;

/// Internal trait to maintain consistency between builder types
#[allow(unused)]
pub(crate) trait Builder<'a>
: Into<Set<'a>> 
+ Into<Cow<'a, Set<'a>>>
+ IntoIterator<Item=Self>
+ UnionWith<'a>
+ ToQuery<'a> {
}

/// Provides methods to build all the various types of OverpassQL [Set]s.
pub struct SetBuilder;

mod filter;
pub use filter::*;

mod union;
pub use union::*;

mod query;
pub use query::*;

use std::borrow::Cow;
use crate::Set;

pub trait Builder<'a>
: Into<Set<'a>> 
+ Into<Cow<'a, Set<'a>>>
+ UnionWith<'a>
+ ToQuery<'a> {
}

pub struct SetBuilder;

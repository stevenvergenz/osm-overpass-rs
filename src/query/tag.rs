use std::{
    fmt::Write, hash::Hash,
};
use crate::{OverpassQL, OverpassQLError, SaniStr};
#[cfg(doc)]
use crate::FilterSet;

/// The set of conditions that an element's tags may or may not satisfy. Used to evaluate the contents of [FilterSet]s.
/// 
/// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#By_tag_.28has-kv.29)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagFilter<'a> {
    /// Satisfied if a tag with this name/key is present.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Exists)
    Exists(SaniStr<'a>),
    /// Satisfied if a tag with this name/key is not present.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Not_exists)
    NotExists(SaniStr<'a>),
    /// Satisfied if a tag with this name/key exists and has the given value.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Equals_.28.3D.2C_.21.3D.29)
    Equals(SaniStr<'a>, SaniStr<'a>),
    /// Satisfied if a tag with this name/key does not exist, or has a value other than the given one.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Equals_.28.3D.2C_.21.3D.29)
    NotEquals(SaniStr<'a>, SaniStr<'a>),
    /// Satisfied if a tag with this name/key exists, and has a value that matches the given regular expression.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Value_matches_regular_expression_.28.7E.2C_.21.7E.29)
    Matches(SaniStr<'a>, SaniStr<'a>),
    /// Satisfied if a tag with this name/key does not exist, or has a value that does not match the given regular expression.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Value_matches_regular_expression_.28.7E.2C_.21.7E.29)
    NotMatches(SaniStr<'a>, SaniStr<'a>),
    /// Satisfied if a tag whose name/key matches the first regular expression has a value that matches the second regular expression.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Key.2Fvalue_matches_regular_expression_.28.7E.22key_regex.22.7E.22value_regex.22.29)
    NameValueMatches(SaniStr<'a>, SaniStr<'a>),
}

impl OverpassQL for TagFilter<'_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            Self::Exists(n) => write!(f, "[{n}]"),
            Self::NotExists(n) => write!(f, "[!{n}]"),
            Self::Equals(n, v) => write!(f, "[{n}={v}]"),
            Self::NotEquals(n, v) => write!(f, "[{n}!={v}]"),
            Self::Matches(n, v) => write!(f, "[{n}~{v}]"),
            Self::NotMatches(n, v) => write!(f, "[{n}!~{v}]"),
            Self::NameValueMatches(n, v) => write!(f, "[~{n}~{v}]"),
        }?;
        Ok(())
    }
}

impl<'a> TagFilter<'a> {
    /// Create a new [Exists](Self::Exists) variant.
    pub fn exists(name: &'a str) -> Self {
        Self::Exists(SaniStr(name))
    }
    /// Create a new [NotExists](Self::NotExists) variant.
    pub fn not_exists(name: &'a str) -> Self {
        Self::NotExists(SaniStr(name))
    }
    /// Create a new [Equals](Self::Equals) variant.
    pub fn equals(name: &'a str, value: &'a str) -> Self {
        Self::Equals(SaniStr(name), SaniStr(value))
    }
    /// Create a new [NotEquals](Self::NotEquals) variant.
    pub fn not_equals(name: &'a str, value: &'a str) -> Self {
        Self::NotEquals(SaniStr(name), SaniStr(value))
    }
    /// Create a new [Matches](Self::Matches) variant.
    pub fn matches(name: &'a str, value_pat: &'a str) -> Self {
        Self::Matches(SaniStr(name), SaniStr(value_pat))
    }
    /// Create a new [NotMatches](Self::NotMatches) variant.
    pub fn not_matches(name: &'a str, value_pat: &'a str) -> Self {
        Self::NotMatches(SaniStr(name), SaniStr(value_pat))
    }
    /// Create a new [NameValueMatches](Self::NameValueMatches) variant.
    pub fn name_value_matches(name_pat: &'a str, value_pat: &'a str) -> Self {
        Self::NameValueMatches(SaniStr(name_pat), SaniStr(value_pat))
    }
}


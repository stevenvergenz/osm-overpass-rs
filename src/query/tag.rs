use std::{
    fmt::Write, hash::Hash,
};
use crate::{OverpassQL, OverpassQLError, SaniStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagFilter<'a> {
    Exists(SaniStr<'a>),
    NotExists(SaniStr<'a>),
    Equals(SaniStr<'a>, SaniStr<'a>),
    NotEquals(SaniStr<'a>, SaniStr<'a>),
    Matches(SaniStr<'a>, SaniStr<'a>),
    NotMatches(SaniStr<'a>, SaniStr<'a>),
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
    pub fn exists(name: &'a str) -> Self {
        Self::Exists(SaniStr(name))
    }
    pub fn not_exists(name: &'a str) -> Self {
        Self::NotExists(SaniStr(name))
    }
    pub fn equals(name: &'a str, value: &'a str) -> Self {
        Self::Equals(SaniStr(name), SaniStr(value))
    }
    pub fn not_equals(name: &'a str, value: &'a str) -> Self {
        Self::NotEquals(SaniStr(name), SaniStr(value))
    }
    pub fn matches(name: &'a str, value_pat: &'a str) -> Self {
        Self::Matches(SaniStr(name), SaniStr(value_pat))
    }
    pub fn not_matches(name: &'a str, value_pat: &'a str) -> Self {
        Self::NotMatches(SaniStr(name), SaniStr(value_pat))
    }
    pub fn name_value_matches(name_pat: &'a str, value_pat: &'a str) -> Self {
        Self::NameValueMatches(SaniStr(name_pat), SaniStr(value_pat))
    }
}


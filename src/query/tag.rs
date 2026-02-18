use std::{
    fmt::{Display, Formatter, Result as FResult, Write},
    hash::{Hash, Hasher},
};
use crate::{OverpassQL, OverpassQLError, QuerySet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TagMatcher<'a> {
    Exact(&'a str),
    NotExact(&'a str),
    Matching(&'a str),
    NotMatching(&'a str),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TagName<'a>(TagMatcher<'a>);

impl OverpassQL for TagName<'_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self.0 {
            TagMatcher::Exact(n) => write!(f, r#""{n}""#).map_err(OverpassQLError::from),
            TagMatcher::NotExact(n) => write!(f, r#"!"{n}""#).map_err(OverpassQLError::from),
            TagMatcher::Matching(n) => write!(f, r#"~"{n}""#).map_err(OverpassQLError::from),
            TagMatcher::NotMatching(_) => panic!("Tag names cannot be matched with inverted regular expressions"),
        }
    }
}

impl Display for TagName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagValue<'a>(TagMatcher<'a>);

impl OverpassQL for TagValue<'_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self.0 {
            TagMatcher::Exact(n) => write!(f, r#"="{n}""#).map_err(OverpassQLError::from),
            TagMatcher::NotExact(n) => write!(f, r#"!="{n}""#).map_err(OverpassQLError::from),
            TagMatcher::Matching(n) => write!(f, r#"~"{n}""#).map_err(OverpassQLError::from),
            TagMatcher::NotMatching(n) => write!(f, r#"!~"{n}""#).map_err(OverpassQLError::from),
        }
    }
}

impl Display for TagValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagFilter<'a> {
    pub name: TagName<'a>,
    pub value: Option<TagValue<'a>>,
}

impl<'a> TagFilter<'a> {
    pub fn new(name: TagName<'a>, value: TagValue<'a>) -> Self {
        Self { name, value: Some(value) }
    }

    pub fn name(name: TagName<'a>) -> Self {
        Self { name, value: None }
    }
}

impl OverpassQL for TagFilter<'_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        write!(f, "[").map_err(OverpassQLError::from)?;
        self.name.fmt_oql(f).map_err(OverpassQLError::from)?;
        if let Some(value) = self.value {
            value.fmt_oql(f).map_err(OverpassQLError::from)?;
        }
        write!(f, "]").map_err(OverpassQLError::from)
    }
}

impl Display for TagFilter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

impl Hash for TagFilter<'_> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.name.hash(state)
    }
}

// value exact match
impl<'a> QuerySet<'a> {
    pub fn with_tag_value(mut self, tag: &'a str, tag_value: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::new(
            TagName(TagMatcher::Exact(tag)), 
            TagValue(TagMatcher::Exact(tag_value)),
        ));
        self
    }

    pub fn with_tag_values(mut self, tags: impl IntoIterator<Item=(&'a str, &'a str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self = self.with_tag_value(k, v);
        }
        self
    }

    pub fn without_tag_value(mut self, tag: &'a str, tag_value: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::new(
            TagName(TagMatcher::Exact(tag)), 
            TagValue(TagMatcher::NotExact(tag_value)),
        ));
        self
    }

    pub fn without_tag_values(mut self, tags: impl IntoIterator<Item=(&'a str, &'a str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self = self.without_tag_value(k, v);
        }
        self
    }
}

// exists
impl<'a> QuerySet<'a> {
    pub fn with_tag(mut self, tag: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::name(TagName(TagMatcher::Exact(tag))));
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item=&'a str>) -> Self {
        for tag in tags.into_iter() {
            self = self.with_tag(tag);
        }
        self
    }

    pub fn without_tag(mut self, tag: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::name(TagName(TagMatcher::NotExact(tag))));
        self
    }

    pub fn without_tags(mut self, tags: impl IntoIterator<Item=&'a str>) -> Self {
        for tag in tags.into_iter() {
            self = self.without_tag(tag);
        }
        self
    }
}

// value regex match
impl<'a> QuerySet<'a> {
    pub fn with_tag_value_matching(mut self, tag: &'a str, value_re: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::new(
            TagName(TagMatcher::Exact(tag)), 
            TagValue(TagMatcher::Matching(value_re)),
        ));
        self
    }

    pub fn with_tag_values_matching(mut self, tags: impl IntoIterator<Item=(&'a str, &'a str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self = self.with_tag_value_matching(k, v);
        }
        self
    }

    pub fn without_tag_value_matching(mut self, tag: &'a str, value_re: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::new(
            TagName(TagMatcher::Exact(tag)), 
            TagValue(TagMatcher::NotMatching(value_re)),
        ));
        self
    }

    pub fn without_tag_values_matching(mut self, tags: impl IntoIterator<Item=(&'a str, &'a str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self = self.without_tag_value_matching(k, v);
        }
        self
    }

    pub fn with_matching_tag_value_matching(mut self, tag_re: &'a str, value_re: &'a str) -> Self {
        self.tag_filters.insert(TagFilter::new(
            TagName(TagMatcher::Matching(tag_re)), 
            TagValue(TagMatcher::Matching(value_re)),
        ));
        self
    }

    pub fn with_matching_tag_values_matching(mut self, tags: impl IntoIterator<Item=(&'a str, &'a str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self = self.with_matching_tag_value_matching(k, v);
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let n = TagName(TagMatcher::Exact("type"));
        assert_eq!(n.to_oql().as_str(), r#""type""#);
        let n = TagName(TagMatcher::NotExact("type"));
        assert_eq!(n.to_oql().as_str(), r#"!"type""#);
        let n = TagName(TagMatcher::Matching("type"));
        assert_eq!(n.to_oql().as_str(), r#"~"type""#);
    }

    #[test]
    fn value() {
        let n = TagValue(TagMatcher::Exact("route"));
        assert_eq!(n.to_oql().as_str(), r#"="route""#);
        let n = TagValue(TagMatcher::NotExact("route"));
        assert_eq!(n.to_oql().as_str(), r#"!="route""#);
        let n = TagValue(TagMatcher::Matching("route"));
        assert_eq!(n.to_oql().as_str(), r#"~"route""#);
        let n = TagValue(TagMatcher::NotMatching("route"));
        assert_eq!(n.to_oql().as_str(), r#"!~"route""#);
    }

    #[test]
    fn filter() {
        // equals
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::Exact("route")),
        );
        assert_eq!(f.to_oql().as_str(), r#"["type"="route"]"#);

        // not equals
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::NotExact("route")),
        );
        assert_eq!(f.to_oql().as_str(), r#"["type"!="route"]"#);

        // exists
        let f = TagFilter::name(TagName(TagMatcher::Exact("type")));
        assert_eq!(f.to_oql().as_str(), r#"["type"]"#);

        // not exists
        let f = TagFilter::name(TagName(TagMatcher::NotExact("type")));
        assert_eq!(f.to_oql().as_str(), r#"[!"type"]"#);

        // value matches
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::Matching("route")),
        );
        assert_eq!(f.to_oql().as_str(), r#"["type"~"route"]"#);

        // value not matches
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::NotMatching("route")),
        );
        assert_eq!(f.to_oql().as_str(), r#"["type"!~"route"]"#);

        // double matches
        let f = TagFilter::new(
            TagName(TagMatcher::Matching("type")), 
            TagValue(TagMatcher::Matching("route")),
        );
        assert_eq!(f.to_oql().as_str(), r#"[~"type"~"route"]"#);
    }
}

use std::{
    fmt::{Display, Result as FResult, Write},
    hash::{Hash, Hasher},
};
use crate::{Overpass, QuerySet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TagMatcher<'a> {
    Exact(&'a str),
    NotExact(&'a str),
    Matching(&'a str),
    NotMatching(&'a str),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TagName<'a>(TagMatcher<'a>);

impl Overpass for TagName<'_> {
    fn fmt_op(&self, f: &mut impl Write) -> FResult {
        match self.0 {
            TagMatcher::Exact(n) => write!(f, r#""{n}""#),
            TagMatcher::NotExact(n) => write!(f, r#"!"{n}""#),
            TagMatcher::Matching(n) => write!(f, r#"~"{n}""#),
            TagMatcher::NotMatching(_) => panic!("Tag names cannot be matched with inverted regular expressions"),
        }
    }
}

impl Display for TagName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_op(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagValue<'a>(TagMatcher<'a>);

impl Overpass for TagValue<'_> {
    fn fmt_op(&self, f: &mut impl Write) -> FResult {
        match self.0 {
            TagMatcher::Exact(n) => write!(f, r#"="{n}""#),
            TagMatcher::NotExact(n) => write!(f, r#"!="{n}""#),
            TagMatcher::Matching(n) => write!(f, r#"~"{n}""#),
            TagMatcher::NotMatching(n) => write!(f, r#"!~"{n}""#),
        }
    }
}

impl Display for TagValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_op(f)
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

impl Overpass for TagFilter<'_> {
    fn fmt_op(&self, f: &mut impl Write) -> FResult {
        write!(f, "[")?;
        self.name.fmt_op(f)?;
        if let Some(value) = self.value {
            value.fmt_op(f)?;
        }
        write!(f, "]")
    }
}

impl Display for TagFilter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_op(f)
    }
}

impl Hash for TagFilter<'_> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.name.hash(state)
    }
}

impl<'i, 'f> QuerySet<'i, 'f> {
    pub fn with_tag_values(mut self, tags: impl IntoIterator<Item=(&'f str, &'f str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self.tag_filters.insert(TagFilter::new(
                TagName(TagMatcher::Exact(k)), 
                TagValue(TagMatcher::Exact(v)),
            ));
        }
        self
    }

    pub fn without_tag_values(mut self, tags: impl IntoIterator<Item=(&'f str, &'f str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self.tag_filters.insert(TagFilter::new(
                TagName(TagMatcher::Exact(k)), 
                TagValue(TagMatcher::NotExact(v)),
            ));
        }
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item=&'f str>) -> Self {
        for tag in tags.into_iter() {
            self.tag_filters.insert(TagFilter::name(TagName(TagMatcher::Exact(tag))));
        }
        self
    }

    pub fn without_tags(mut self, tags: impl IntoIterator<Item=&'f str>) -> Self {
        for tag in tags.into_iter() {
            self.tag_filters.insert(TagFilter::name(TagName(TagMatcher::NotExact(tag))));
        }
        self
    }

    pub fn with_tag_values_matching(mut self, tags: impl IntoIterator<Item=(&'f str, &'f str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self.tag_filters.insert(TagFilter::new(
                TagName(TagMatcher::Exact(k)), 
                TagValue(TagMatcher::Matching(v)),
            ));
        }
        self
    }

    pub fn without_tag_values_matching(mut self, tags: impl IntoIterator<Item=(&'f str, &'f str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self.tag_filters.insert(TagFilter::new(
                TagName(TagMatcher::Exact(k)), 
                TagValue(TagMatcher::NotMatching(v)),
            ));
        }
        self
    }

    pub fn with_matching_tag_values_matching(mut self, tags: impl IntoIterator<Item=(&'f str, &'f str)>) -> Self {
        for (k, v) in tags.into_iter() {
            self.tag_filters.insert(TagFilter::new(
                TagName(TagMatcher::Matching(k)), 
                TagValue(TagMatcher::Matching(v)),
            ));
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
        assert_eq!(n.to_overpass().as_str(), r#""type""#);
        let n = TagName(TagMatcher::NotExact("type"));
        assert_eq!(n.to_overpass().as_str(), r#"!"type""#);
        let n = TagName(TagMatcher::Matching("type"));
        assert_eq!(n.to_overpass().as_str(), r#"~"type""#);
    }

    #[test]
    fn value() {
        let n = TagValue(TagMatcher::Exact("route"));
        assert_eq!(n.to_overpass().as_str(), r#"="route""#);
        let n = TagValue(TagMatcher::NotExact("route"));
        assert_eq!(n.to_overpass().as_str(), r#"!="route""#);
        let n = TagValue(TagMatcher::Matching("route"));
        assert_eq!(n.to_overpass().as_str(), r#"~"route""#);
        let n = TagValue(TagMatcher::NotMatching("route"));
        assert_eq!(n.to_overpass().as_str(), r#"!~"route""#);
    }

    #[test]
    fn filter() {
        // equals
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::Exact("route")),
        );
        assert_eq!(f.to_overpass().as_str(), r#"["type"="route"]"#);

        // not equals
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::NotExact("route")),
        );
        assert_eq!(f.to_overpass().as_str(), r#"["type"!="route"]"#);

        // exists
        let f = TagFilter::name(TagName(TagMatcher::Exact("type")));
        assert_eq!(f.to_overpass().as_str(), r#"["type"]"#);

        // not exists
        let f = TagFilter::name(TagName(TagMatcher::NotExact("type")));
        assert_eq!(f.to_overpass().as_str(), r#"[!"type"]"#);

        // value matches
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::Matching("route")),
        );
        assert_eq!(f.to_overpass().as_str(), r#"["type"~"route"]"#);

        // value not matches
        let f = TagFilter::new(
            TagName(TagMatcher::Exact("type")), 
            TagValue(TagMatcher::NotMatching("route")),
        );
        assert_eq!(f.to_overpass().as_str(), r#"["type"!~"route"]"#);

        // double matches
        let f = TagFilter::new(
            TagName(TagMatcher::Matching("type")), 
            TagValue(TagMatcher::Matching("route")),
        );
        assert_eq!(f.to_overpass().as_str(), r#"[~"type"~"route"]"#);
    }
}
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Result as FResult, Write},
    hash::{Hash, Hasher},
};
use crate::{
    Bbox, Overpass, query::ReferencedByFilter
};
use super::{
    TagFilter, TagName, TagValue,
};

#[derive(Debug, Clone, Copy)]
pub enum QuerySetType {
    Node,
    Way,
    Relation,
    Any,
    NodeOrWay,
    NodeOrRelation,
    WayOrRelation,
    Derived,
    Area,
}

impl Overpass for QuerySetType {
    fn fmt_op(&self, f: &mut impl Write) -> FResult {
        match self {
            Self::Node => write!(f, "node"),
            Self::Way => write!(f, "way"),
            Self::Relation => write!(f, "relation"),
            Self::Any => write!(f, "nwr"),
            Self::NodeOrWay => write!(f, "nw"),
            Self::NodeOrRelation => write!(f, "nr"),
            Self::WayOrRelation => write!(f, "wr"),
            Self::Derived => write!(f, "derived"),
            Self::Area => write!(f, "area"),
        }
    }
}

impl Display for QuerySetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_op(f)
    }
}

#[derive(Debug, Clone)]
pub struct QuerySet<'i, 'f> {
    pub output_type: QuerySetType,
    pub input: Option<&'i QuerySet<'i, 'f>>,
    pub tag_filters: HashSet<TagFilter<'f>>,
    pub bbox_filter: Option<Bbox>,
    pub ref_filters: HashSet<ReferencedByFilter<'i, 'f>>,
}

impl Default for QuerySet<'_, '_> {
    fn default() -> Self {
        Self {
            output_type: QuerySetType::Any,
            input: None,
            tag_filters: HashSet::new(),
            bbox_filter: None,
            ref_filters: HashSet::new(),
        }
    }
}

impl<'i, 'f> QuerySet<'i, 'f> {
    pub fn from(mut self, input: &'i QuerySet<'i, 'f>) -> Self {
        self.input = Some(input);
        self
    }
}

/// constructors
impl<'i, 'f> QuerySet<'i, 'f> {
    pub fn nodes() -> Self {
        Self {
            output_type: QuerySetType::Node,
            ..Default::default()
        }
    }

    pub fn ways() -> Self {
        Self {
            output_type: QuerySetType::Way,
            ..Default::default()
        }
    }
    
    pub fn relations() -> Self {
        Self {
            output_type: QuerySetType::Relation,
            ..Default::default()
        }
    }
    
    pub fn any_type() -> Self {
        Self {
            output_type: QuerySetType::Any,
            ..Default::default()
        }
    }
    
    pub fn nodes_or_ways() -> Self {
        Self {
            output_type: QuerySetType::NodeOrWay,
            ..Default::default()
        }
    }
    
    pub fn nodes_or_relations() -> Self {
        Self {
            output_type: QuerySetType::NodeOrRelation,
            ..Default::default()
        }
    }
    
    pub fn ways_or_relations() -> Self {
        Self {
            output_type: QuerySetType::WayOrRelation,
            ..Default::default()
        }
    }
    
    pub fn derived() -> Self {
        Self {
            output_type: QuerySetType::Derived,
            ..Default::default()
        }
    }
    
    pub fn area() -> Self {
        Self {
            output_type: QuerySetType::Area,
            ..Default::default()
        }
    }
}

impl Overpass for QuerySet<'_, '_> {
    fn fmt_op(&self, f: &mut impl Write) -> FResult {
        self.output_type.fmt_op(f)?;

        if let Some(bbox) = self.bbox_filter {
            write!(f, "(")?;
            bbox.fmt_op(f)?;
            write!(f, ")")?;
        }

        for filter in self.tag_filters.iter() {
            filter.fmt_op(f)?;
        }

        Ok(())
    }
}

impl Display for QuerySet<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_op(f)
    }
}

impl PartialEq for QuerySet<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for QuerySet<'_, '_> {}

impl Hash for QuerySet<'_, '_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let s = QuerySet::nodes().with_tag_values([("public_transport", "platform")]);
        assert_eq!(s.to_overpass().as_str(), r#"node["public_transport"="platform"]"#);
    }
}
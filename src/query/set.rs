use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Display, Formatter, Result as FResult, Write},
    hash::{Hash, Hasher},
};
use crate::{
    Bbox, OverpassQL, OverpassQLError,
};
use super::{
    TagFilter,
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

impl OverpassQL for QuerySetType {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            Self::Node => write!(f, "node").map_err(OverpassQLError::from),
            Self::Way => write!(f, "way").map_err(OverpassQLError::from),
            Self::Relation => write!(f, "relation").map_err(OverpassQLError::from),
            Self::Any => write!(f, "nwr").map_err(OverpassQLError::from),
            Self::NodeOrWay => write!(f, "nw").map_err(OverpassQLError::from),
            Self::NodeOrRelation => write!(f, "nr").map_err(OverpassQLError::from),
            Self::WayOrRelation => write!(f, "wr").map_err(OverpassQLError::from),
            Self::Derived => write!(f, "derived").map_err(OverpassQLError::from),
            Self::Area => write!(f, "area").map_err(OverpassQLError::from),
        }
    }
}

impl Display for QuerySetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

#[derive(Debug, Clone)]
pub struct QuerySet<'i, 'f> {
    pub content_type: QuerySetType,
    pub input: Option<&'i QuerySet<'i, 'f>>,
    pub tag_filters: HashSet<TagFilter<'f>>,
    pub bbox_filter: Option<Bbox>,
    pub id: RefCell<Option<String>>,
}

impl Default for QuerySet<'_, '_> {
    fn default() -> Self {
        Self {
            content_type: QuerySetType::Any,
            input: None,
            tag_filters: HashSet::new(),
            bbox_filter: None,
            id: RefCell::new(None),
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
            content_type: QuerySetType::Node,
            ..Default::default()
        }
    }

    pub fn ways() -> Self {
        Self {
            content_type: QuerySetType::Way,
            ..Default::default()
        }
    }
    
    pub fn relations() -> Self {
        Self {
            content_type: QuerySetType::Relation,
            ..Default::default()
        }
    }
    
    pub fn any_type() -> Self {
        Self {
            content_type: QuerySetType::Any,
            ..Default::default()
        }
    }
    
    pub fn nodes_or_ways() -> Self {
        Self {
            content_type: QuerySetType::NodeOrWay,
            ..Default::default()
        }
    }
    
    pub fn nodes_or_relations() -> Self {
        Self {
            content_type: QuerySetType::NodeOrRelation,
            ..Default::default()
        }
    }
    
    pub fn ways_or_relations() -> Self {
        Self {
            content_type: QuerySetType::WayOrRelation,
            ..Default::default()
        }
    }
    
    pub fn derived() -> Self {
        Self {
            content_type: QuerySetType::Derived,
            ..Default::default()
        }
    }
    
    pub fn area() -> Self {
        Self {
            content_type: QuerySetType::Area,
            ..Default::default()
        }
    }
}

impl OverpassQL for QuerySet<'_, '_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        self.content_type.fmt_oql(f).map_err(OverpassQLError::from)?;

        if let Some(input) = self.input
        && let Some(id) = input.id.borrow().as_ref() {
            write!(f, ".{id}").map_err(OverpassQLError::from)?;
        }

        if let Some(bbox) = self.bbox_filter {
            write!(f, "(").map_err(OverpassQLError::from)?;
            bbox.fmt_oql(f)?;
            write!(f, ")")?;
        }

        for filter in self.tag_filters.iter() {
            filter.fmt_oql(f)?;
        }

        if let Some(id) = self.id.borrow().as_ref() {
            write!(f, "->.{id}").map_err(OverpassQLError::from)?;
        }

        Ok(())
    }
}

impl Display for QuerySet<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
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
        assert_eq!(s.to_oql().as_str(), r#"node["public_transport"="platform"]"#);
    }
}
use std::{
    borrow::Cow,
    collections::HashSet,
    fmt::{Display, Formatter, Result as FResult, Write},
    hash::{Hash, Hasher},
};
use crate::{
    Bbox, OverpassQL, OverpassQLError, Query, Namer, TagFilter,
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
pub struct QuerySet<'a> {
    pub content_type: QuerySetType,
    pub input: Option<Box<Cow<'a, QuerySet<'a>>>>,
    pub id_filters: HashSet<i64>,
    pub tag_filters: HashSet<TagFilter<'a>>,
    pub bbox_filter: Option<Bbox>,
}

impl Default for QuerySet<'_> {
    fn default() -> Self {
        Self {
            content_type: QuerySetType::Any,
            input: None,
            id_filters: HashSet::new(),
            tag_filters: HashSet::new(),
            bbox_filter: None,
        }
    }
}

impl<'a> QuerySet<'a> {
    pub fn from(mut self, input: impl Into<Cow<'a, QuerySet<'a>>>) -> Self {
        self.input = Some(Box::new(input.into()));
        self
    }

    pub fn to_query(self) -> Query<'a> {
        Query {
            query_set: self,
            ..Default::default()
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id_filters.insert(id);
        self
    }

    pub fn with_ids(mut self, ids: impl IntoIterator<Item=i64>) -> Self {
        for i in ids.into_iter() {
            self = self.with_id(i);
        }
        self
    }
}

/// constructors
impl QuerySet<'_> {
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

impl<'a> QuerySet<'a> {
    fn fmt_filters(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        if self.id_filters.len() > 0 {
            let mut iter = self.id_filters.iter();
            write!(f, "(id:{}", iter.next().unwrap())?;
            for i in iter {
                write!(f, ",{i}")?;
            }
            write!(f, ")")?;
        }

        if let Some(bbox) = self.bbox_filter {
            write!(f, "(").map_err(OverpassQLError::from)?;
            bbox.fmt_oql(f)?;
            write!(f, ")")?;
        }

        for filter in self.tag_filters.iter() {
            filter.fmt_oql(f)?;
        }

        Ok(())
    }

    pub(crate) fn fmt_oql_named<'b>(&'b self,
        f: &mut impl Write,
        namer: &'b mut Namer<'a, 'b>,
    ) -> Result<(), OverpassQLError>
    where 'a: 'b {
        self.content_type.fmt_oql(f).map_err(OverpassQLError::from)?;

        if let Some(input) = &self.input
        && let Some(name) = namer.get_or_assign(input) {
            write!(f, ".{name}").map_err(OverpassQLError::from)?;
        }

        self.fmt_filters(f)?;

        if let Some(name) = namer.get_or_assign(self) {
            write!(f, "->.{name}").map_err(OverpassQLError::from)?;
        }
        
        Ok(())
    }
}

impl OverpassQL for QuerySet<'_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        self.content_type.fmt_oql(f).map_err(OverpassQLError::from)?;

        self.fmt_filters(f)?;

        Ok(())
    }
}

impl Display for QuerySet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

impl PartialEq for QuerySet<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for QuerySet<'_> {}

impl Hash for QuerySet<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}

impl<'a> Into<Cow<'a, QuerySet<'a>>> for QuerySet<'a> {
    fn into(self) -> Cow<'a, QuerySet<'a>> {
        Cow::Owned(self)
    }
}

impl<'a> Into<Cow<'a, QuerySet<'a>>> for &'a QuerySet<'a> {
    fn into(self) -> Cow<'a, QuerySet<'a>> {
        Cow::Borrowed(self)
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

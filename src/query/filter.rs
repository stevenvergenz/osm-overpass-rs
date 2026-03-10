use std::{
    borrow::Cow,
    collections::{hash_set::IntoIter, HashSet},
    fmt::Write,
};
use crate::{
    Bbox, Namer, OverpassQLError, OverpassQLNamed, OverpassQL, RecurseFilter, Set, TagFilter
};

#[derive(Debug, Clone, Copy, Default)]
pub enum FilterType {
    Node,
    Way,
    Relation,
    #[default]
    Any,
    NodeOrWay,
    NodeOrRelation,
    WayOrRelation,
    Derived,
    Area,
}

impl OverpassQL for FilterType {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            Self::Node => write!(f, "node")?,
            Self::Way => write!(f, "way")?,
            Self::Relation => write!(f, "relation")?,
            Self::Any => write!(f, "nwr")?,
            Self::NodeOrWay => write!(f, "nw")?,
            Self::NodeOrRelation => write!(f, "nr")?,
            Self::WayOrRelation => write!(f, "wr")?,
            Self::Derived => write!(f, "derived")?,
            Self::Area => write!(f, "area")?,
        }
        Ok(())
    }
}

impl<'a> Into<Set<'a>> for FilterType {
    fn into(self) -> Set<'a> {
        Set::Filter(FilterSet { filter_type: self, ..Default::default() })
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterSet<'a> {
    pub filter_type: FilterType,
    pub inputs: HashSet<Cow<'a, Set<'a>>>,
    pub id_filters: HashSet<i64>,
    pub tag_filters: HashSet<TagFilter<'a>>,
    pub bbox_filter: Option<Bbox>,
    pub recurse_filters: HashSet<RecurseFilter<'a>>,
}

impl<'a> OverpassQLNamed<'a> for FilterSet<'a> {
    fn fmt_oql_named<'b, 'c>(&'b self, f: &mut impl Write, namer: &mut Namer<'a, 'c>)
    -> Result<(), OverpassQLError>
    where 'b: 'c {
        self.filter_type.fmt_oql(f)?;

        for input in &self.inputs {
            if let Some(name) = namer.get_or_assign(input) {
                write!(f, ".{name}")?;
            }
        }

        if self.id_filters.len() > 0 {
            let mut iter = self.id_filters.iter();
            write!(f, "(id:{}", iter.next().unwrap())?;
            for i in iter {
                write!(f, ",{i}")?;
            }
            write!(f, ")")?;
        }

        if let Some(bbox) = &self.bbox_filter {
            write!(f, "(")?;
            bbox.fmt_oql(f)?;
            write!(f, ")")?;
        }

        for filter in &self.tag_filters {
            filter.fmt_oql(f)?;
        }

        for filter in &self.recurse_filters {
            filter.fmt_oql_named(f, namer)?;
        }
        
        Ok(())
    }
}

impl<'a> FilterSet<'a> {
    pub fn dependencies(&self) -> IntoIter<&Set<'a>> {
        self.inputs.iter().map(|i| i.as_ref())
            .chain(self.recurse_filters.iter().map(|r| r.input()))
            .collect::<HashSet<_>>().into_iter()
    }
}

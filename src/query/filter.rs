use crate::{
    Bbox, Namer, OverpassQL, OverpassQLError, OverpassQLNamed, RecurseFilter, Set, TagFilter,
};
#[cfg(doc)]
use crate::{Node, Relation, Way};
use std::{
    borrow::Cow,
    collections::{HashSet, hash_set::IntoIter},
    fmt::Write,
};

/// The type of element selected by a [FilterSet].
#[derive(Debug, Clone, Copy, Default)]
pub enum FilterType {
    /// [Node]s
    Node,
    /// [Way]s
    Way,
    /// [Relation]s
    Relation,
    /// Any element type
    #[default]
    Any,
    NodeOrWay,
    NodeOrRelation,
    WayOrRelation,
    // Derived,
    /// Ways or relations that the server determines represent a two-dimensional area and not just
    /// a line. [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Areas)
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
            // Self::Derived => write!(f, "derived")?,
            Self::Area => write!(f, "area")?,
        }
        Ok(())
    }
}

impl<'a> Into<Set<'a>> for FilterType {
    fn into(self) -> Set<'a> {
        Set::Filter(FilterSet {
            filter_type: self,
            ..Default::default()
        })
    }
}

/// A subtype of [Set] that contains elements that satisfy the specified criteria.
#[derive(Debug, Clone, Default)]
pub struct FilterSet<'a> {
    /// The type(s) of elements eligible to be in this set.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#The_Query_Statement)
    pub filter_type: FilterType,

    /// Only elements common to all of these sets are eligible to be in this set.
    /// An empty collection means all elements are eligible.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#By_input_set_.28.setname.29)
    pub inputs: HashSet<Cow<'a, Set<'a>>>,

    /// Only elements with one of these identifiers are eligible to be in this set.
    /// An empty collection means identifiers are not considered.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#By_element_id)
    pub id_filters: HashSet<i64>,

    /// Only elements whose tags satisfy all these filters are eligible to be in this set.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#By_tag_.28has-kv.29)
    pub tag_filters: HashSet<TagFilter<'a>>,

    /// Only elements that lie within these bounds are eligible to be in this set.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Bounding_box)
    pub bbox_filter: Option<Bbox>,

    /// Only elements with these relationships are eligible to be in this set.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_.28n.2C_w.2C_r.2C_bn.2C_bw.2C_br.29)
    pub recurse_filters: HashSet<RecurseFilter<'a>>,
}

impl<'a> OverpassQLNamed<'a> for FilterSet<'a> {
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c,
    {
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
    /// The sets that must be defined before this set.
    pub fn dependencies(&self) -> IntoIter<&Set<'a>> {
        self.inputs
            .iter()
            .map(|i| i.as_ref())
            .chain(self.recurse_filters.iter().map(|r| r.input()))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

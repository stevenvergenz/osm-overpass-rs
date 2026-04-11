use crate::{Bbox, Namer, OverpassQL, OverpassQLError, OverpassQLNamed, Set};
use std::{borrow::Cow, fmt::Write};

/// The amount of detail to be included in [Query]-matched [Element]s.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Output_format_.28out%3A.29)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QueryVerbosity {
    /// Include no elements at all, only the number of elements of each type.
    Count,

    /// Include the element type, ID, coordinates/members, and tags.
    #[default]
    Body,

    /// Include the element type and ID only.
    Ids,

    /// Include the element type, ID, and coordinates/members only.
    Skeleton,

    /// Include the element type, ID, and tags only.
    Tags,
    //Meta,
}

impl OverpassQL for QueryVerbosity {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            Self::Body => write!(f, "body"),
            Self::Count => write!(f, "count"),
            Self::Ids => write!(f, "ids"),
            Self::Tags => write!(f, "tags"),
            Self::Skeleton => write!(f, "skel"),
            //Self::Meta => write!(f, "out meta;"),
        }?;
        Ok(())
    }
}

/// The amount of geometry that should be computed for elements.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QueryGeometry {
    /// No geometry will be computed.
    #[default]
    None,

    /// The bounding box centerpoint will be computed for ways and relations.
    Center,

    /// The bounding box will be computed for ways and relations.
    Bbox,

    /// The full coordinate set will be computed for ways and relation members.
    Geometry,
}

impl OverpassQL for QueryGeometry {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            Self::None => Ok(()),
            Self::Center => write!(f, "center"),
            Self::Bbox => write!(f, "bb"),
            Self::Geometry => write!(f, "geom"),
        }?;
        Ok(())
    }
}

/// The order in which set elements are output.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortOrder {
    /// Sort by ascending element id (default).
    #[default]
    Asc,
    /// Sort by quadtile index; this is roughly geographical and significantly faster than order by ids.
    Quadtile,
}

impl OverpassQL for SortOrder {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Quadtile => write!(f, "qt"),
        }?;
        Ok(())
    }
}

/// Configuration for how a set's elements should be represented.
#[derive(Debug, Clone, Default)]
pub struct QueryOutput<'a> {
    /// Adjust the amount of detail included in returned [Element](crate::Element)s.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Output_format_.28out%3A.29)
    pub verbosity: QueryVerbosity,

    /// Adjust how much element geometry is returned from the server.
    pub geo: QueryGeometry,

    /// Only return geometry within these bounds.
    pub bbox: Option<Bbox>,

    /// How to sort the returned elements.
    pub sort: SortOrder,

    /// Only output up to this many elements.
    pub limit: Option<usize>,

    /// The [Set] of [Element](crate::Element)s to be returned when this query is [evaluate](crate::Overpass::evaluate)d.
    pub set: Cow<'a, Set<'a>>,
}

impl<'a> OverpassQLNamed<'a> for QueryOutput<'a> {
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c,
    {
        write!(f, ".{} out", namer.get_or_assign(&self.set))?;
        if self.verbosity != QueryVerbosity::default() {
            write!(f, " ")?;
            self.verbosity.fmt_oql(f)?;
        }
        if self.geo != QueryGeometry::default() {
            write!(f, " ")?;
            self.geo.fmt_oql(f)?;
        }
        if let Some(bbox) = &self.bbox {
            write!(f, " ")?;
            bbox.fmt_oql(f)?;
        }
        write!(f, ";")?;
        Ok(())
    }
}

use crate::{OverpassQL, OverpassQLError, Point};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// A geographic bounding box defined by two latitudes and two longitudes. Used to distinguish
/// points inside and outside the box.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Bbox {
    /// The latitude of the southern edge of the bounding box.
    #[serde(alias = "minlat")]
    pub south: f64,

    /// The longitude of the western edge of the bounding box.
    #[serde(alias = "minlon")]
    pub west: f64,

    /// The latitude of the northern edge of the bounding box.
    #[serde(alias = "maxlat")]
    pub north: f64,

    /// The longitude of the eastern edge of the bounding box.
    #[serde(alias = "maxlon")]
    pub east: f64,
}

impl Bbox {
    /// Create a new bounding box from the given coordinate bounds.
    pub fn new(south: f64, west: f64, north: f64, east: f64) -> Self {
        Self {
            south,
            west,
            north,
            east,
        }
    }
}

impl OverpassQL for Bbox {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        let Self {
            south,
            west,
            north,
            east,
        } = self;
        write!(f, "{south},{west},{north},{east}")?;
        Ok(())
    }
}

impl From<Point> for Bbox {
    fn from(value: Point) -> Self {
        Self {
            south: value.lat,
            north: value.lat,
            east: value.lon,
            west: value.lon,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fmt() {
        let b = Bbox::new(1f64, 2f64, 3f64, 4f64);
        assert_eq!(b.to_oql().as_str(), "1,2,3,4");
    }

    // #[test]
    // fn query() {
    //     let s = Set::all_types().within_bounds(Bbox::new(1.5, 2.5, 3.5, 4.5));
    //     assert_eq!(s.to_oql().as_str(), r#"nwr(1.5,2.5,3.5,4.5)"#);
    // }
}

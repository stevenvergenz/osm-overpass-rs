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

impl Default for Bbox {
    fn default() -> Self {
        Self {
            south: f64::INFINITY,
            west: f64::INFINITY,
            north: -f64::INFINITY,
            east: -f64::INFINITY,
        }
    }
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

    /// True if the bounding box has been initialized, and has a non-negative area.
    pub fn is_valid(&self) -> bool {
        self.east >= self.west && self.north >= self.south
    }

    /// Calculate the center of the bounding box.
    pub fn center(&self) -> Point {
        Point::new((self.north + self.south) / 2., (self.east + self.west) / 2.)
    }

    /// The northeast corner of the bounding box.
    pub fn northeast(&self) -> Point {
        Point::new(self.north, self.east)
    }

    /// The northwest corner of the bounding box.
    pub fn northwest(&self) -> Point {
        Point::new(self.north, self.west)
    }

    /// The southeast corner of the bounding box.
    pub fn southeast(&self) -> Point {
        Point::new(self.south, self.east)
    }

    /// The southwest corner of the bounding box.
    pub fn southwest(&self) -> Point {
        Point::new(self.south, self.west)
    }

    /// Whether a point is contained by the bounding box.
    pub fn contains(&self, p: Point) -> bool {
        p.lat <= self.north && p.lat >= self.south && p.lon <= self.east && p.lon >= self.west
    }

    /// Modify the box to contain a point. Returns true if the bounds change.
    pub fn contain(&mut self, p: Point) -> bool {
        let mut updated = false;

        if p.lat > self.north {
            self.north = p.lat;
            updated = true;
        }
        if p.lat < self.south {
            self.south = p.lat;
            updated = true;
        }
        if p.lon > self.east {
            self.east = p.lon;
            updated = true;
        }
        if p.lon < self.west {
            self.west = p.lon;
            updated = true;
        }

        updated
    }

    /// Returns a new bounding box created from expanding this box to contain the given point.
    pub fn containing(&self, p: Point) -> Self {
        let mut new = self.clone();
        new.contain(p);
        new
    }
}

impl FromIterator<Point> for Bbox {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut bbox = Self::default();
        for i in iter {
            bbox.contain(i);
        }
        bbox
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
            west: value.lon,
            north: value.lat,
            east: value.lon,
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

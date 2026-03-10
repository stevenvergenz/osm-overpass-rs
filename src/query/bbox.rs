use std::fmt::Write;
use crate::{OverpassQL, OverpassQLError};

/// A geographic bounding box defined by two latitudes and two longitudes. Used to distinguish
/// points inside and outside the box.
#[derive(Debug, Clone, Copy)]
pub struct Bbox {
    /// The latitude of the southern edge of the bounding box.
    pub south: f64,
    /// The longitude of the western edge of the bounding box.
    pub west: f64,
    /// The latitude of the northern edge of the bounding box.
    pub north: f64,
    /// The longitude of the eastern edge of the bounding box.
    pub east: f64,
}

impl Bbox {
    pub fn new(south: f64, west: f64, north: f64, east: f64) -> Self {
        Self { south, west, north, east }
    }
}

impl OverpassQL for Bbox {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        let Self { south, west, north, east } = self;
        write!(f, "{south},{west},{north},{east}")?;
        Ok(())
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

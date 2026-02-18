use std::fmt::{Display, Formatter, Result as FResult, Write};
use crate::{OverpassQL, OverpassQLError, QuerySet};

#[derive(Debug, Clone, Copy)]
pub struct Bbox {
    pub south: f64,
    pub west: f64,
    pub north: f64,
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
        write!(f, "{south},{west},{north},{east}").map_err(OverpassQLError::from)
    }
}

impl Display for Bbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

impl QuerySet<'_> {
    pub fn within_bounds(mut self, bounds: Bbox) -> Self {
        self.bbox_filter = Some(bounds);
        self
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

    #[test]
    fn query() {
        let s = QuerySet::any_type().within_bounds(Bbox::new(1.5, 2.5, 3.5, 4.5));
        assert_eq!(s.to_oql().as_str(), r#"nwr(1.5,2.5,3.5,4.5)"#);
    }
}

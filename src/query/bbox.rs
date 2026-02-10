use std::fmt::{Display, Result as FResult, Write};
use crate::{Overpass, QuerySet};

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

impl Overpass for Bbox {
    fn fmt_op(&self, f: &mut impl Write) -> FResult {
        let Self { south, west, north, east } = self;
        write!(f, "{south},{west},{north},{east}")
    }
}

impl Display for Bbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FResult {
        self.fmt_op(f)
    }
}

impl<'i, 'f> QuerySet<'i, 'f> {
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
        assert_eq!(b.to_overpass().as_str(), "1,2,3,4");
    }

    #[test]
    fn query() {
        let s = QuerySet::any_type().within_bounds(Bbox::new(1.5, 2.5, 3.5, 4.5));
        assert_eq!(s.to_overpass().as_str(), r#"nwr(1.5,2.5,3.5,4.5)"#);
    }
}
use std::{
    fmt::{Display, Formatter, Result as FResult},
    hash::Hash,
};
use crate::{Overpass, QuerySet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReferencedByFilter<'i, 'f> {
    Ways(&'i QuerySet<'i, 'f>),
}

impl<'i, 'f> QuerySet<'i, 'f> {
    pub fn referenced_by_ways<'a, 'b>(mut self, way_set: &'a QuerySet<'a, 'b>) -> Self
    where 'a: 'i, 'b: 'f {
        self.ref_filters.insert(ReferencedByFilter::Ways(way_set));
        self
    }
}
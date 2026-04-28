use std::borrow::Cow;

use crate::{RecurseSet, Set, SetBuilder, SetBuilderCommon};

/// A builder struct for a [RecurseSet].
#[derive(Debug, Clone)]
pub struct RecurseSetBuilder<'a>(pub Set<'a>);

/// Methods that return recurse set builders.
impl<'a> SetBuilder<'a> {
    /// Returns elements that are referenced by an input element.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_down_.28.3E.29).
    pub fn recurse_down(
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> RecurseSetBuilder<'a> {
        RecurseSetBuilder(RecurseSet::down(set).into())
    }

    /// Returns elements that are referenced by an input element, and members of any found relations.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_down_relations_.28.3E.3E.29).
    pub fn recurse_down_relations(
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> RecurseSetBuilder<'a> {
        RecurseSetBuilder(RecurseSet::down_relations(set).into())
    }

    /// Returns elements that reference an input element.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_up_.28.3C.29).
    pub fn recurse_up(
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> RecurseSetBuilder<'a> {
        RecurseSetBuilder(RecurseSet::up(set).into())
    }

    /// Returns elements that reference an input element, and relations that reference any found element.
    /// See the [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Recurse_up_relations_.28.3C.3C.29).
    pub fn recurse_up_relations(
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> RecurseSetBuilder<'a> {
        RecurseSetBuilder(RecurseSet::up_relations(set).into())
    }
}

impl<'a> SetBuilderCommon<'a> for RecurseSetBuilder<'a> {
    type Inner = RecurseSet<'a>;
    fn inner(&mut self) -> &mut Self::Inner {
        match &mut self.0 {
            Set::Recurse(r) => r,
            _ => panic!("bad variant"),
        }
    }
}

impl<'a> Into<Set<'a>> for RecurseSetBuilder<'a> {
    fn into(self) -> Set<'a> {
        self.0
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for RecurseSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        self.0.into()
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for &'a RecurseSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        self.as_ref().into()
    }
}

impl<'a> IntoIterator for RecurseSetBuilder<'a> {
    type Item = Self;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

impl<'a> AsRef<Set<'a>> for RecurseSetBuilder<'a> {
    fn as_ref(&self) -> &Set<'a> {
        &self.0
    }
}

impl<'a> AsMut<Set<'a>> for RecurseSetBuilder<'a> {
    fn as_mut(&mut self) -> &mut Set<'a> {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[tokio::test]
    async fn recurse() {
        let stops = SetBuilder::nodes_or_ways()
            .with_tag_value("public_transport", "platform");
        let routes = SetBuilder::recurse_up(&stops)
            .filter_to(FilterType::Relation)
            .with_tag_value("public_transport", "route");

        OverpassServer::default()
            .evaluate(
                SetBuilder::union([&stops, &routes])
                    .to_query()
                    .search_bbox(Bbox {
                        north: 47.667,
                        south: 47.553,
                        east: -122.201,
                        west: -122.461,
                    })
                    .as_ref(),
            )
            .await
            .expect("Failed request");
    }
}

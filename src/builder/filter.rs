use crate::{
    Bbox, Builder, FilterSet, FilterType, RecurseFilter, SaniStr, Set,
    SetBuilder, TagFilter,
};
#[cfg(doc)]
use crate::{Node, Relation, Way};
use std::borrow::Cow;

/// A convenient builder API for [FilterSet].
pub struct FilterSetBuilder<'a>(
    /// The set being configured.
    pub FilterSet<'a>,
);

impl<'a> Builder<'a> for FilterSetBuilder<'a> {}

impl<'a> Into<Set<'a>> for FilterSetBuilder<'a> {
    fn into(self) -> Set<'a> {
        self.0.into()
    }
}

impl<'a> Into<Cow<'a, Set<'a>>> for FilterSetBuilder<'a> {
    fn into(self) -> Cow<'a, Set<'a>> {
        Cow::Owned(self.into())
    }
}

impl<'a> IntoIterator for FilterSetBuilder<'a> {
    type Item = FilterSetBuilder<'a>;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

/// Methods to create new [FilterSet]s.
impl SetBuilder {
    /// Start a new filter set containing [Node]s, i.e. [FilterType::Node].
    pub fn nodes<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Node,
            ..Default::default()
        })
    }

    /// Start a new filter set containing [Way]s, i.e. [FilterType::Way].
    pub fn ways<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Way,
            ..Default::default()
        })
    }

    /// Start a new filter set containing [Relation]s, i.e. [FilterType::Relation].
    pub fn relations<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Relation,
            ..Default::default()
        })
    }

    /// Start a new filter set containing any element type, i.e. [FilterType::Any].
    pub fn any_type<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Any,
            ..Default::default()
        })
    }

    /// Start a new filter set containing [Node]s or [Way]s, i.e. [FilterType::NodeOrWay].
    pub fn nodes_or_ways<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::NodeOrWay,
            ..Default::default()
        })
    }

    /// Start a new filter set containing [Node]s or [Relation]s, i.e. [FilterType::NodeOrRelation].
    pub fn nodes_or_relations<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::NodeOrRelation,
            ..Default::default()
        })
    }

    /// Start a new filter set containing [Way]s or [Relation]s, i.e. [FilterType::WayOrRelation].
    pub fn ways_or_relations<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::WayOrRelation,
            ..Default::default()
        })
    }

    /*
    pub fn derived<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Derived,
            ..Default::default()
        })
    }
    */

    /// Start a new filter set containing elements identified as areas, i.e. [FilterType::Area].
    pub fn areas<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Area,
            ..Default::default()
        })
    }
}

impl<'a> FilterSetBuilder<'a> {
    pub fn from<T>(mut self, sets: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Cow<'a, Set<'a>>>,
    {
        for i in sets.into_iter() {
            self.0.inputs.insert(i.into());
        }
        self
    }

    /// Restrict this set to only elements with the given identifier. See [FilterSet::id_filters].
    pub fn with_id(mut self, id: i64) -> Self {
        self.0.id_filters.clear();
        self.0.id_filters.insert(id);
        self
    }

    /// Restrict this set to only elements with the given identifiers. See [FilterSet::id_filters].
    pub fn with_ids(mut self, ids: impl IntoIterator<Item = i64>) -> Self {
        self.0.id_filters.clear();
        for id in ids {
            self.0.id_filters.insert(id);
        }
        self
    }

    /// Restrict this set to only elements contained by the given bounding box. See [FilterSet::bbox_filter].
    pub fn within_bounds(mut self, bbox: impl Into<Bbox>) -> Self {
        self.0.bbox_filter = Some(bbox.into());
        self
    }

    /// Restrict this set to only elements that have a tag of the given name. See [TagFilter::Exists].
    pub fn with_tag(mut self, tag: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::exists(tag));
        self
    }

    /// Restrict this set to only elements that do not have a tag of the given name. See [TagFilter::NotExists].
    pub fn without_tag(mut self, tag: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::not_exists(tag));
        self
    }

    /// Restrict this set to only elements that have the given tag with the given value. See [TagFilter::Equals].
    pub fn with_tag_value(mut self, tag: &'a str, value: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::equals(tag, value));
        self
    }

    /// Restrict this set to only elements that do not have the given tag, or have a value different from the given value.
    /// See [TagFilter::NotEquals].
    pub fn without_tag_value(mut self, tag: &'a str, value: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::not_equals(tag, value));
        self
    }

    /// Restrict this set to only elements that have the given tag with a value that matches the given regular expression.
    /// See [TagFilter::Matches].
    pub fn with_tag_value_matching(
        mut self,
        tag: &'a str,
        value_pat: &'a str,
    ) -> Self {
        self.0
            .tag_filters
            .insert(TagFilter::matches(tag, value_pat));
        self
    }

    /// Restrict this set to only elements that do not have the given tag, or have a value that does not match the given
    /// regular expression.
    /// See [TagFilter::NotMatches].
    pub fn without_tag_value_matching(
        mut self,
        tag: &'a str,
        value_pat: &'a str,
    ) -> Self {
        self.0
            .tag_filters
            .insert(TagFilter::not_matches(tag, value_pat));
        self
    }

    /// Restrict this set to only elements that have a tag matching the given regular expression,
    /// with a value that matches the given regular expression.
    /// See [TagFilter::NameValueMatches].
    pub fn with_tag_name_and_value_matching(
        mut self,
        tag_pat: &'a str,
        value_pat: &'a str,
    ) -> Self {
        self.0
            .tag_filters
            .insert(TagFilter::name_value_matches(tag_pat, value_pat));
        self
    }

    /// Restrict this set to only elements that have tags of the given names. See [TagFilter::Exists].
    pub fn with_tags(
        mut self,
        tags: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        for i in tags {
            self.0.tag_filters.insert(TagFilter::Exists(SaniStr(i)));
        }
        self
    }

    /// Restrict this set to only elements that do not have tags of the given names. See [TagFilter::NotExists].
    pub fn without_tags(
        mut self,
        tags: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        for i in tags {
            self.0.tag_filters.insert(TagFilter::NotExists(SaniStr(i)));
        }
        self
    }

    /// Restrict this set to only elements that have the given tags with the given values. See [TagFilter::Equals].
    pub fn with_tag_values(
        mut self,
        tags: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0
                .tag_filters
                .insert(TagFilter::Equals(SaniStr(n), SaniStr(v)));
        }
        self
    }

    /// Restrict this set to only elements that do not have the given tags, or have values different from the given values.
    /// See [TagFilter::NotEquals].
    pub fn without_tag_values(
        mut self,
        tags: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0
                .tag_filters
                .insert(TagFilter::NotEquals(SaniStr(n), SaniStr(v)));
        }
        self
    }

    /// Restrict this set to only elements that have the given tags with values that matches the given regular expressions.
    /// See [TagFilter::Matches].
    pub fn with_tag_values_matching(
        mut self,
        tags: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0
                .tag_filters
                .insert(TagFilter::Matches(SaniStr(n), SaniStr(v)));
        }
        self
    }

    /// Restrict this set to only elements that do not have the given tags, or have values that does not match the given
    /// regular expressions.
    /// See [TagFilter::NotMatches].
    pub fn without_tag_values_matching(
        mut self,
        tags: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0
                .tag_filters
                .insert(TagFilter::NotMatches(SaniStr(n), SaniStr(v)));
        }
        self
    }

    /// Restrict this set to only elements that have tags matching the given regular expressions,
    /// with values that matches the given regular expressions.
    /// See [TagFilter::NameValueMatches].
    pub fn with_tag_names_and_values_matching(
        mut self,
        tags: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0
                .tag_filters
                .insert(TagFilter::NameValueMatches(SaniStr(n), SaniStr(v)));
        }
        self
    }

    /// Restrict this set to only [Node]s that are members of a [Way] in the given set.
    /// See [RecurseFilter::WithinWays].
    pub fn within_ways(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::WithinWays { input: set.into() });
        self
    }

    /// Restrict this set to only elements that are members of a [Relation] in the given set.
    /// See [RecurseFilter::WithinRelations].
    pub fn within_relations(
        mut self,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::WithinRelations {
                input: set.into(),
                role: None,
            });
        self
    }

    /// Restrict this set to only elements that are members of a [Relation] in the given set, and that membership has
    /// the given role.
    /// See [RecurseFilter::WithinRelations].
    pub fn within_relations_with_role(
        mut self,
        role: &'a str,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::WithinRelations {
                input: set.into(),
                role: Some(SaniStr(role)),
            });
        self
    }

    /// Restrict this set to only elements that have a [Node] in this set as a member.
    /// See [RecurseFilter::ContainingNodes].
    pub fn containing_nodes(
        mut self,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::ContainingNodes {
                input: set.into(),
                role: None,
            });
        self
    }

    /// Restrict this set to only elements that have a [Node] in this set as a member, and that
    /// membership has the given role.
    /// See [RecurseFilter::ContainingNodes].
    pub fn containing_nodes_with_role(
        mut self,
        role: &'a str,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::ContainingNodes {
                input: set.into(),
                role: Some(SaniStr(role)),
            });
        self
    }

    /// Restrict this set to only elements that have a [Way] in this set as a member.
    /// See [RecurseFilter::ContainingWays].
    pub fn containing_ways(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::ContainingWays {
                input: set.into(),
                role: None,
            });
        self
    }

    /// Restrict this set to only elements that have a [Way] in this set as a member, and that
    /// membership has the given role.
    /// See [RecurseFilter::ContainingWays].
    pub fn containing_ways_with_role(
        mut self,
        role: &'a str,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::ContainingWays {
                input: set.into(),
                role: Some(SaniStr(role)),
            });
        self
    }

    /// Restrict this set to only elements that have a [Relation] in this set as a member.
    /// See [RecurseFilter::ContainingRelations].
    pub fn containing_relations(
        mut self,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::ContainingRelations {
                input: set.into(),
                role: None,
            });
        self
    }

    /// Restrict this set to only elements that have a [Relation] in this set as a member, and that
    /// membership has the given role.
    /// See [RecurseFilter::ContainingRelations].
    pub fn containing_relations_with_role(
        mut self,
        role: &'a str,
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0
            .recurse_filters
            .insert(RecurseFilter::ContainingRelations {
                input: set.into(),
                role: Some(SaniStr(role)),
            });
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _all_nodes_from() {
        let _ = SetBuilder::nodes().from(SetBuilder::nodes());

        let set: Set = SetBuilder::ways().into();
        let _ = SetBuilder::nodes().from([&set]);
    }
}

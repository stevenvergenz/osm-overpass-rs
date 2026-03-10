use std::borrow::Cow;
use crate::{Bbox, FilterSet, FilterType, RecurseFilter, SaniStr, Set, SetBuilder, TagFilter, Builder};

pub struct FilterSetBuilder<'a>(
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

/// Methods to create new [FilterSet]s.
impl SetBuilder {
    pub fn all_nodes<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Node,
            ..Default::default()
        })
    }

    pub fn nodes_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Node,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }

    pub fn all_ways<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Way,
            ..Default::default()
        })
    }

    pub fn ways_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Way,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn all_relations<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Relation,
            ..Default::default()
        })
    }

    pub fn relations_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Relation,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn any_type<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Any,
            ..Default::default()
        })
    }

    pub fn any_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Any,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn all_nodes_or_ways<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::NodeOrWay,
            ..Default::default()
        })
    }

    pub fn nodes_or_ways_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::NodeOrWay,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn all_nodes_or_relations<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::NodeOrRelation,
            ..Default::default()
        })
    }

    pub fn nodes_or_relations_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::NodeOrRelation,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn all_ways_or_relations<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::WayOrRelation,
            ..Default::default()
        })
    }

    pub fn ways_or_relations_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::WayOrRelation,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn all_derived<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Derived,
            ..Default::default()
        })
    }

    pub fn derived_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Derived,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
    
    pub fn all_areas<'a>() -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Area,
            ..Default::default()
        })
    }

    pub fn areas_from<'a, T>(sets: impl IntoIterator<Item=T>)
    -> FilterSetBuilder<'a>
    where T: Into<Cow<'a, Set<'a>>> {
        FilterSetBuilder(FilterSet {
            filter_type: FilterType::Area,
            inputs: sets.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        })
    }
}

impl<'a> FilterSetBuilder<'a> {
    pub fn with_id(mut self, id: i64) -> Self {
        self.0.id_filters.insert(id);
        self
    }

    pub fn with_ids(mut self, ids: impl IntoIterator<Item=i64>) -> Self {
        for id in ids {
            self.0.id_filters.insert(id);
        }
        self
    }

    pub fn within_bounds(mut self, bbox: impl Into<Bbox>) -> Self {
        self.0.bbox_filter = Some(bbox.into());
        self
    }

    pub fn with_tag(mut self, tag: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::exists(tag));
        self
    }

    pub fn without_tag(mut self, tag: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::not_exists(tag));
        self
    }

    pub fn with_tag_value(mut self, tag: &'a str, value: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::equals(tag, value));
        self
    }

    pub fn without_tag_value(mut self, tag: &'a str, value: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::not_equals(tag, value));
        self
    }

    pub fn with_tag_value_matching(mut self, tag: &'a str, value_pat: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::matches(tag, value_pat));
        self
    }

    pub fn without_tag_value_matching(mut self, tag: &'a str, value_pat: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::not_matches(tag, value_pat));
        self
    }

    pub fn with_tag_name_and_value_matching(mut self, tag_pat: &'a str, value_pat: &'a str) -> Self {
        self.0.tag_filters.insert(TagFilter::name_value_matches(tag_pat, value_pat));
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item=&'a str>) -> Self {
        for i in tags {
            self.0.tag_filters.insert(TagFilter::Exists(SaniStr(i)));
        }
        self
    }

    pub fn without_tags(mut self, tags: impl IntoIterator<Item=&'a str>) -> Self {
        for i in tags {
            self.0.tag_filters.insert(TagFilter::NotExists(SaniStr(i)));
        }
        self
    }

    pub fn with_tag_values(
        mut self, 
        tags: impl IntoIterator<Item=(&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0.tag_filters.insert(TagFilter::Equals(SaniStr(n), SaniStr(v)));
        }
        self
    }

    pub fn without_tag_values(
        mut self, 
        tags: impl IntoIterator<Item=(&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0.tag_filters.insert(TagFilter::NotEquals(SaniStr(n), SaniStr(v)));
        }
        self
    }

    pub fn with_tag_values_matching(
        mut self, 
        tags: impl IntoIterator<Item=(&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0.tag_filters.insert(TagFilter::Matches(SaniStr(n), SaniStr(v)));
        }
        self
    }

    pub fn without_tag_values_matching(
        mut self, 
        tags: impl IntoIterator<Item=(&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0.tag_filters.insert(TagFilter::NotMatches(SaniStr(n), SaniStr(v)));
        }
        self
    }

    pub fn with_tag_names_and_values_matching(
        mut self, 
        tags: impl IntoIterator<Item=(&'a str, &'a str)>,
    ) -> Self {
        for (n, v) in tags {
            self.0.tag_filters.insert(TagFilter::NameValueMatches(SaniStr(n), SaniStr(v)));
        }
        self
    }

    pub fn within_ways(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::WithinWays { input: set.into() });
        self
    }

    pub fn within_relations(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::WithinRelations { input: set.into(), role: None });
        self
    }

    pub fn within_relations_with_role(
        mut self, 
        role: &'a str, 
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::WithinRelations { input: set.into(), role: Some(SaniStr(role)) });
        self
    }

    pub fn containing_nodes(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::ContainingNodes { input: set.into(), role: None });
        self
    }

    pub fn containing_nodes_with_role(
        mut self, 
        role: &'a str, 
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::ContainingNodes { input: set.into(), role: Some(SaniStr(role)) });
        self
    }

    pub fn containing_ways(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::ContainingWays { input: set.into(), role: None });
        self
    }

    pub fn containing_ways_with_role(
        mut self, 
        role: &'a str, 
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::ContainingWays { input: set.into(), role: Some(SaniStr(role)) });
        self
    }

    pub fn containing_relations(mut self, set: impl Into<Cow<'a, Set<'a>>>) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::ContainingRelations { input: set.into(), role: None });
        self
    }

    pub fn containing_relations_with_role(
        mut self, 
        role: &'a str, 
        set: impl Into<Cow<'a, Set<'a>>>,
    ) -> Self {
        self.0.recurse_filters.insert(RecurseFilter::ContainingRelations { input: set.into(), role: Some(SaniStr(role)) });
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _all_nodes_from() {
        let _ = SetBuilder::nodes_from([SetBuilder::all_nodes()]);

        let set: Set = SetBuilder::all_ways().into();
        let _ = SetBuilder::nodes_from([&set]);
    }
}

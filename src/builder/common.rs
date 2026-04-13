use crate::{
    DifferenceSetBuilder, FilterSet, FilterSetBuilder, FilterType, OutputBuilder, Query, QueryBuilder, QueryOutput, Set, UnionSetBuilder
};
use std::{borrow::Cow, collections::HashSet};

/// Trait to maintain consistency between builder types.
pub trait SetBuilderCommon<'a>:
    Into<Set<'a>>
    + Into<Cow<'a, Set<'a>>>
    + IntoIterator<Item = Self>
    + AsRef<Set<'a>>
    + AsMut<Set<'a>>
where
    Self: 'a,
    &'a Self: Into<Cow<'a, Set<'a>>>,
{
    /// The specific set variant for this builder.
    type Inner: Into<Set<'a>> + TryFrom<Set<'a>>;

    /// A mutable reference to the inner set type.
    fn inner(&mut self) -> &mut Self::Inner;

    /// Create a new set with elements from this set that meet certain criteria.
    fn filter(self, filter_type: FilterType) -> FilterSetBuilder<'a> {
        FilterSetBuilder(FilterSet {
            filter_type,
            inputs: HashSet::from([self.into()]),
            ..Default::default()
        }.into())
    }
    /// Create a new set with all elements from both this and another set.
    fn union_with(
        self,
        other: impl Into<Cow<'a, Set<'a>>>,
    ) -> UnionSetBuilder<'a> {
        UnionSetBuilder::from_iter([self.into(), other.into()])
    }

    /// Exclude a set's elements from this set.
    fn without(
        self,
        exclude: impl Into<Cow<'a, Set<'a>>>,
    ) -> DifferenceSetBuilder<'a> {
        DifferenceSetBuilder::new(self, exclude)
    }

    /// Start configuring output options for this set.
    fn to_output(self) -> OutputBuilder<'a> {
        OutputBuilder(vec![QueryOutput {
            set: self.into(),
            ..Default::default()
        }])
    }

    /// Start configuring query options for this set.
    fn to_query(self) -> QueryBuilder<'a> {
        QueryBuilder(Query {
            outputs: self.to_output().0,
            ..Default::default()
        })
    }
}

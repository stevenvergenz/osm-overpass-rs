use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};
use chrono::{DateTime, Utc};
use crate::{OverpassQLUnnamed, OverpassQLError, Set, Bbox, OverpassQLNamed, Namer};

/// The amount of detail to be included in [Query]-matched [crate::Element]s.
/// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Output_format_.28out%3A.29)
#[derive(Debug, Clone, Copy, Default)]
pub enum QueryVerbosity {
    //Count,

    /// Include the element type, ID, coordinates/members, and tags.
    #[default]
    Body,

    /// Include the element type and ID only.
    Ids,

    /// Include the element type, ID, and coordinates/members only.
    Skeleton,

    /// Include the element type, ID, and tags only.
    Tags,

    //Meta,
}

impl OverpassQLUnnamed for QueryVerbosity {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        match self {
            //Self::Count => write!(f, "out count;"),
            Self::Body => write!(f, "out;"),
            Self::Ids => write!(f, "out ids;"),
            Self::Tags => write!(f, "out tags;"),
            Self::Skeleton => write!(f, "out skel;"),
            //Self::Meta => write!(f, "out meta;"),
        }?;
        Ok(())
    }
}

/// The root type of this API. It serializes into a complete Overpass QL query via [OverpassQLUnnamed::to_oql].
#[derive(Debug, Default)]
pub struct Query<'a> {
    /// The length of time in seconds after which the server will abort the query.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#timeout%3A)
    pub timeout_s: Option<u32>,

    /// The maximum allowed memory for the query in bytes RAM on the server, beyond which the server will abort the query.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Element_limit_.28maxsize%3A.29)
    pub max_size: Option<u32>,

    /// Apply the query only to elements within the region defined by the bounding box.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Global_bounding_box_.28bbox.29)
    pub search_bbox: Option<Bbox>,

    /// Query the state of the map as of the given date/time.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Date)
    pub as_of_date: Option<DateTime<Utc>>,

    /// Query only elements created/modified between the first date/time, and the second date/time if supplied, or now if not.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Difference_between_two_dates_.28diff.29)
    pub diff: Option<(DateTime<Utc>, Option<DateTime<Utc>>)>,

    /// Adjust the amount of detail included in returned [crate::Element]s.
    /// [wiki](https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL#Output_format_.28out%3A.29)
    pub verbosity: QueryVerbosity,

    /// The [Set] of [crate::Element]s to be returned when this query is [crate::Overpass::evaluate]d.
    pub set: Set<'a>,
}

impl<'a> From<Set<'a>> for Query<'a> {
    fn from(value: Set<'a>) -> Self {
        Self {
            set: value,
            ..Default::default()
        }
    }
}

impl<'a> AsRef<Query<'a>> for Query<'a> {
    fn as_ref(&self) -> &Query<'a> {
        self
    }
}

fn resolve_ordering<'a, 'b>(query_set: &'b Set<'a>)
-> Result<Vec<&'b Set<'a>>, OverpassQLError>
where 'a: 'b {
    // for {k: [v]}, v must be defined before k
    let mut refs = evaluate_refs(query_set, HashMap::new());

    // for {k: [v]}, k must be defined before v
    let mut back_refs = HashMap::new();
    for (a, b) in refs.iter() {
        for c in b {
            back_refs.entry(*c).or_insert(HashSet::new()).insert(*a);
        }
    }

    let mut output = vec![];

    while refs.len() > 0 {
        // find sets with no dependencies
        let next_outputs = refs
            .extract_if(|_, v| v.len() == 0)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        // fail if there aren't any
        if next_outputs.len() == 0 {
            return Err(OverpassQLError::CircularReference);
        }

        for next in next_outputs {
            // output them first
            output.push(next);

            // take them out of any reference list that contains them
            if let Some(next_refs) = back_refs.remove(next) {
                for referent in next_refs.iter() {
                    refs.get_mut(referent).unwrap().remove(next);
                }
            }
            
        }
    }

    Ok(output)
}

fn evaluate_refs<'a, 'b>(
    set: &'b Set<'a>, 
    mut refs: HashMap<&'b Set<'a>, HashSet<&'b Set<'a>>>,
) -> HashMap<&'b Set<'a>, HashSet<&'b Set<'a>>>
where 'a: 'b {
    let deps = refs.entry(set).or_insert(HashSet::new());
    let fresh = set.dependencies()
        .filter(|s| deps.insert(s))
        .collect::<Vec<_>>();

    for i in fresh {
        refs = evaluate_refs(i, refs);
    }

    refs
}

impl<'a> OverpassQLUnnamed for Query<'a> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        if let Some(d) = self.timeout_s {
            write!(f, "[timeout:{}]", d)?;
        }
        if let Some(s) = self.max_size {
            write!(f, "[maxsize:{s}]")?;
        }
        if let Some(bbox) = self.search_bbox {
            write!(f, "[bbox:")?;
            bbox.fmt_oql(f)?;
            write!(f, "]")?;
        }
        if let Some(d) = self.as_of_date {
            write!(f, r#"[date:"{d}"]"#)?;
        }
        if let Some((a, mayb)) = self.diff {
            if let Some(b) = mayb {
                write!(f, r#"[diff:"{a}","{b}"]"#)?;
            } else {
                write!(f, r#"[diff:"{a}"]"#)?;
            }
        }
        write!(f, "[out:json];")?;

        let mut namer = Namer::new(&self.set);
        for set in resolve_ordering(&self.set)? {
            set.fmt_oql_named(f, &mut namer)?;
            write!(f, ";")?;
        }

        self.verbosity.fmt_oql(f)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use super::*;
    use crate::{FilterSet, FilterType, TagFilter};

    #[test]
    fn resolve_ordering() {
        let q1 = Set::Filter(FilterSet {
            filter_type: FilterType::NodeOrWay,
            tag_filters: HashSet::from([
                TagFilter::equals("public_transport", "platform"),
            ]),
            ..Default::default()
        });
        let q2 = Set::Filter(FilterSet {
            filter_type: FilterType::Node,
            inputs: HashSet::from([Cow::Borrowed(&q1)]),
            ..Default::default()
        });

        assert_eq!(super::resolve_ordering(&q2).unwrap(), vec![&q1, &q2]);
    }

    #[test]
    fn fmt_oql() {
        let q1 = Set::Filter(FilterSet {
            filter_type: FilterType::NodeOrWay,
            tag_filters: HashSet::from([
                TagFilter::equals("public_transport", "platform"),
            ]),
            ..Default::default()
        });
        let q2 = Set::Filter(FilterSet {
            filter_type: FilterType::Node,
            inputs: HashSet::from([Cow::Borrowed(&q1)]),
            ..Default::default()
        });
        let q = Query::from(q2);

        assert_eq!(q.to_oql(), vec![
            "[out:json];",
            r#"nw["public_transport"="platform"]->.a;"#,
            "node.a;",
            "out body;"
        ].join(""));
    }
}

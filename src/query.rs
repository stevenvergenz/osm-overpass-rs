mod set;
pub use set::*;

mod filter; 
pub(crate) use filter::*;

mod tag;
pub use tag::*;

mod bbox;
pub use bbox::*;

mod overpassql;
pub use overpassql::*;

mod namer;
pub use namer::*;

mod recurse;
pub use recurse::*;

mod util;
pub use util::*;

use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result as FResult, Write},
};
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy, Default)]
pub enum QueryOutputFormat {
    #[default]
    Body,
    Ids,
    Skeleton,
    Tags,
}

impl OverpassQLUnnamed for QueryOutputFormat {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        let r = match self {
            Self::Body => write!(f, "out body;"),
            Self::Ids => write!(f, "out ids;"),
            Self::Tags => write!(f, "out tags;"),
            Self::Skeleton => write!(f, "out skel;"),
        };
        r.map_err(OverpassQLError::from)
    }
}
impl Display for QueryOutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

#[derive(Debug, Default)]
pub struct Query<'a> {
    pub timeout: Option<Duration>,
    pub max_size: Option<u32>,
    pub global_bbox: Option<Bbox>,
    pub as_of_date: Option<DateTime<Utc>>,
    pub diff: Option<(DateTime<Utc>, Option<DateTime<Utc>>)>,
    pub set: Set<'a>,
    pub output_format: QueryOutputFormat,
}

impl Display for Query<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

impl<'a> From<Set<'a>> for Query<'a> {
    fn from(value: Set<'a>) -> Self {
        Self {
            set: value,
            ..Default::default()
        }
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
            println!("Outputting set {next}");
            // output them first
            output.push(next);

            // take them out of any reference list that contains them
            if let Some(next_refs) = back_refs.remove(next) {
                for referent in next_refs.iter() {
                    println!("Removing reference from {referent}");
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
    let set_refs = match set {
        Set::Filter(f) => f.dependencies(),
    };
    let mut fresh = vec![];

    for i in set_refs {
        if deps.insert(i) {
            fresh.push(i);
        }
    }

    for i in fresh {
        refs = evaluate_refs(i, refs);
    }

    refs
}

impl<'a> OverpassQLUnnamed for Query<'a> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        if let Some(d) = self.timeout {
            write!(f, "[timeout:{}]", d.as_seconds_f32() as u16).map_err(OverpassQLError::from)?;
        }
        if let Some(s) = self.max_size {
            write!(f, "[maxsize:{s}]").map_err(OverpassQLError::from)?;
        }
        if let Some(bbox) = self.global_bbox {
            write!(f, "[bbox:").map_err(OverpassQLError::from)?;
            bbox.fmt_oql(f)?;
            write!(f, "]").map_err(OverpassQLError::from)?;
        }
        if let Some(d) = self.as_of_date {
            write!(f, r#"[date:"{d}"]"#).map_err(OverpassQLError::from)?;
        }
        if let Some((a, mayb)) = self.diff {
            if let Some(b) = mayb {
                write!(f, r#"[diff:"{a}","{b}"]"#).map_err(OverpassQLError::from)?;
            } else {
                write!(f, r#"[diff:"{a}"]"#).map_err(OverpassQLError::from)?;
            }
        }
        write!(f, "[out:json];").map_err(OverpassQLError::from)?;

        let mut namer = Namer::new(&self.set);
        for set in resolve_ordering(&self.set)? {
            set.fmt_oql_named(f, &mut namer)?;
            write!(f, ";").map_err(OverpassQLError::from)?;
        }

        self.output_format.fmt_oql(f)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::*;

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
            inputs: HashSet::from([Box::new(Cow::Borrowed(&q1))]),
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
            inputs: HashSet::from([Box::new(Cow::Borrowed(&q1))]),
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

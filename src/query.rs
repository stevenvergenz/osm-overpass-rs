mod set;
pub use set::*;

mod tag;
pub use tag::*;

mod bbox;
pub use bbox::*;

mod overpassql;
pub use overpassql::*;

mod namer;
pub use namer::*;

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

impl OverpassQL for QueryOutputFormat {
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
pub struct Query<'input, 'filter> {
    pub timeout: Option<Duration>,
    pub max_size: Option<u32>,
    pub global_bbox: Option<Bbox>,
    pub as_of_date: Option<DateTime<Utc>>,
    pub diff: Option<(DateTime<Utc>, Option<DateTime<Utc>>)>,
    pub query_set: QuerySet<'input, 'filter>,
    pub output_format: QueryOutputFormat,
}

fn resolve_ordering<'a, 'input, 'filter>(query_set: &'a QuerySet<'input, 'filter>)
-> Result<Vec<&'a QuerySet<'input, 'filter>>, OverpassQLError>
where 'input: 'a {
    // for {k: [v]}, v must be defined before k
    let mut names = Namer::new();
    let mut refs = evaluate_refs_and_names(query_set, &mut names, HashMap::new());

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


fn evaluate_refs_and_names<'a, 'i, 'f>(
    set: &'a QuerySet<'i, 'f>, 
    names: &mut Namer, 
    mut refs: HashMap<&'a QuerySet<'i, 'f>, HashSet<&'a QuerySet<'i, 'f>>>,
) -> HashMap<&'a QuerySet<'i, 'f>, HashSet<&'a QuerySet<'i, 'f>>>
where 'i: 'a, 'f: 'a {
    let deps = refs.entry(set).or_insert(HashSet::new());
    if let Some(input) = &set.input && deps.insert(input) {
        refs = evaluate_refs_and_names(input, names, refs);
    }
    refs
}

impl<'input, 'filter> OverpassQL for Query<'input, 'filter> {
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

        let mut namer = Namer::new();
        namer.assign(&self.query_set, None);
        for set in resolve_ordering(&self.query_set)? {
            set.fmt_oql_named(f, &mut namer)?;
            write!(f, ";").map_err(OverpassQLError::from)?;
        }

        self.output_format.fmt_oql(f)
    }
}

impl Display for Query<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.fmt_oql(f).map_err(OverpassQLError::into)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_ordering() {
        let q1 = QuerySet::nodes_or_ways().with_tag_values([("public_transport", "platform")]);
        let q2 = QuerySet::nodes().from(&q1);
        assert_eq!(super::resolve_ordering(&q2).unwrap(), vec![&q1, &q2]);
    }

    #[test]
    fn fmt_oql() {
        let s1 = QuerySet::nodes_or_ways().with_tag_values([("public_transport", "platform")]);
        let s2 = QuerySet::nodes().from(&s1);
        assert_eq!(s2.to_query().to_oql(), vec![
            "[out:json]",
            r#"nw["public_transport"="platform"]->.a;"#,
            "node.a;",
            "out;"
        ].join(""));
    }
}

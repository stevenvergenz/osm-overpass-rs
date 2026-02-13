mod set;
pub use set::*;

mod tag;
pub use tag::*;

mod bbox;
pub use bbox::*;

mod overpass;
pub use overpass::*;

use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result as FResult, Write},
};
use chrono::{DateTime, Duration, Utc};


#[derive(Debug)]
pub struct Query<'i, 'f> {
    pub timeout: Option<Duration>,
    pub max_size: Option<u32>,
    pub global_bbox: Option<Bbox>,
    pub as_of_date: Option<DateTime<Utc>>,
    pub diff: Option<(DateTime<Utc>, Option<DateTime<Utc>>)>,
    pub query_set: QuerySet<'i, 'f>,
}

#[derive(Debug, Clone, Default)]
struct NameIterator {
    sequence_index: u32,
}
impl Iterator for NameIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut output = vec![];
        let mut div = self.sequence_index;
        loop {
            output.push(char::from_u32('a' as u32 + (div % 26)).unwrap());
            div /= 26;
            if div == 0 { break; }
        }
        self.sequence_index += 1;
        Some(output.into_iter().rev().collect())
    }
}

impl Query<'_, '_> {
    fn resolve_ordering<'a, 'i, 'f>(
        query_set: &'i QuerySet<'i, 'f>,
    ) -> Result<Vec<&'a QuerySet<'i, 'f>>, OverpassQLError>
    where 'i: 'a {
        // for {k: [v]}, v must be defined before k
        let mut refs = HashMap::new();
        let mut names = NameIterator::default();
        Self::evaluate_refs_and_names(query_set, &mut names, &mut refs);
        dbg!(&refs);

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
        set: &'i QuerySet<'i, 'f>, 
        names: &mut NameIterator, 
        refs: &'a mut HashMap<&'i QuerySet<'i, 'f>, HashSet<&'i QuerySet<'i, 'f>>>,
    ) where 'i: 'a, 'f: 'a {
        let deps = refs.entry(set).or_insert(HashSet::new());
        if let Some(input) = set.input {
            deps.insert(input);
            if input.id.borrow().is_none() {
                input.id.borrow_mut().replace(names.next().unwrap());
                Self::evaluate_refs_and_names(input, names, refs);
            }
        }
    }
}

impl OverpassQL for Query<'_, '_> {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError> {
        write!(f, "[out:json]").map_err(OverpassQLError::from)?;
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

        for set in Self::resolve_ordering(&self.query_set)? {
            set.fmt_oql(f)?;
            write!(f, ";").map_err(OverpassQLError::from)?;
        }

        write!(f, "out;").map_err(OverpassQLError::from)
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
    fn name_iterator() {
        assert_eq!(NameIterator::default().take(12).collect::<Vec<String>>(), vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
        ]);
        assert_eq!(NameIterator::default().step_by(26).take(12).collect::<Vec<String>>(), vec![
            "a", "ba", "ca", "da", "ea", "fa", "ga", "ha", "ia", "ja", "ka", "la",
        ]);
        assert_eq!(NameIterator::default().step_by(26*26).take(12).collect::<Vec<String>>(), vec![
            "a", "baa", "caa", "daa", "eaa", "faa", "gaa", "haa", "iaa", "jaa", "kaa", "laa",
        ]);
    }

    #[test]
    fn resolve_ordering() {
        let q1 = QuerySet::nodes_or_ways().with_tag_values([("public_transport", "platform")]);
        let q2 = QuerySet::nodes().from(&q1);
        assert_eq!(Query::resolve_ordering(&q2).unwrap(), vec![&q1, &q2]);
    }

    #[test]
    fn fmt_oql() {
        let s1 = QuerySet::nodes_or_ways().with_tag_values([("public_transport", "platform")]);
        let s2 = QuerySet::nodes().from(&s1);
        
    }
}
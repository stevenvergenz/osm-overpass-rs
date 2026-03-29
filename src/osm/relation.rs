use serde::{Deserialize, Deserializer, Serialize, de::Visitor};
use crate::{Bbox, ElementCommon, ElementId, Point};

/// Relations are structured collections of objects - nodes, ways, and other relations.
///
/// [wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Relation {
    #[serde(flatten)]
    pub common: ElementCommon,
    pub members: Vec<RelationMember>,
    pub bounds: Option<Bbox>,
    pub center: Option<Point>,
}

/// A reference to another [Node], [Way], or [Relation] from the owning relation, with an optional role.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RelationMember {
    #[serde(flatten)]
    pub id: ElementId,

    /// The role of this element in a relation, if any.
    #[serde(deserialize_with = "skip_empty")]
    pub role: Option<String>,

    pub geometry: Option<Vec<Point>>,
}

fn skip_empty<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_string(OptionalStringVisitor)
}

struct OptionalStringVisitor;
impl<'de> Visitor<'de> for OptionalStringVisitor {
    type Value = Option<String>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "a string")
    }
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        if v.is_empty() {
            Ok(None)
        } else {
            Ok(Some(v.to_string()))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.is_empty() {
            Ok(None)
        } else {
            Ok(Some(v))
        }
    }
}

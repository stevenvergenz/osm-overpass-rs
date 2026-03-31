use crate::ElementId;
use serde::{
    Deserializer,
    de::{Error, SeqAccess, Visitor},
};
use std::fmt::{Formatter, Result as FResult};

pub enum IdVisitor {
    Node,
    Way,
    Relation,
}

impl<'de> Visitor<'de> for IdVisitor {
    type Value = ElementId;

    fn expecting(&self, f: &mut Formatter) -> FResult {
        write!(f, "an integer")
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(match self {
            Self::Node => ElementId::Node(v),
            Self::Way => ElementId::Way(v),
            Self::Relation => ElementId::Relation(v),
        })
    }

    fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }

    fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }

    fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }

    fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }

    fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        self.visit_i64(v as i64)
    }
}

impl IdVisitor {
    pub fn parse_node<'de, D>(deserializer: D) -> Result<ElementId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(Self::Node)
    }
    pub fn parse_way<'de, D>(deserializer: D) -> Result<ElementId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(Self::Way)
    }
    pub fn parse_relation<'de, D>(
        deserializer: D,
    ) -> Result<ElementId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(Self::Relation)
    }
}

pub fn parse_nodes<'de, D>(deserializer: D) -> Result<Vec<ElementId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct IdVecVisitor;
    impl<'de> Visitor<'de> for IdVecVisitor {
        type Value = Vec<ElementId>;

        fn expecting(&self, f: &mut Formatter) -> FResult {
            write!(f, "an array of ints")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut arr = vec![];
            if let Some(size) = seq.size_hint() {
                arr.reserve_exact(size);
            }
            while let Some(i) = seq.next_element()? {
                arr.push(ElementId::Node(i));
            }
            Ok(arr)
        }
    }
    deserializer.deserialize_seq(IdVecVisitor)
}

pub fn skip_empty<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionalStringVisitor;
    impl<'de> Visitor<'de> for OptionalStringVisitor {
        type Value = Option<String>;

        fn expecting(&self, f: &mut Formatter) -> FResult {
            write!(f, "a string")
        }

        fn visit_borrowed_str<E: Error>(
            self,
            v: &'de str,
        ) -> Result<Self::Value, E> {
            if v.is_empty() {
                Ok(None)
            } else {
                Ok(Some(v.to_string()))
            }
        }

        fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
            if v.is_empty() {
                Ok(None)
            } else {
                Ok(Some(v))
            }
        }
    }

    deserializer.deserialize_string(OptionalStringVisitor)
}

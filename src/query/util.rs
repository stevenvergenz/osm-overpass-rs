use std::fmt::{Display, Formatter, Result as FResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SaniStr<'a>(pub &'a str);

impl<'a> From<&'a str> for SaniStr<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl Display for SaniStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        let mut iter = self.0.split('"');
        write!(f, r#""{}"#, iter.next().unwrap())?;
        for i in iter {
            write!(f, r#"\"{i}"#)?;
        }
        write!(f, r#"""#)
    }
}


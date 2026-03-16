use std::fmt::{Display, Formatter, Result as FResult};

/// Handles string sanitization for OverpassQL queries.
/// 
/// Wraps the input string in double-quotes and escapes any double-quotes in the original string.
/// 
/// Example:
/// ```
/// # use overpass_lib::SaniStr;
/// let sani = SaniStr(r#"Dwayne "The Rock" Johnson"#);
/// assert_eq!(sani.to_string(), String::from(r#""Dwayne \"The Rock\" Johnson""#))
/// ```
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


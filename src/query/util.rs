use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FResult},
    sync::LazyLock,
};

/// Handles string sanitization for OverpassQL queries.
///
/// Wraps the input string in double-quotes and escapes any double-quotes in the original string.
///
/// Example:
/// ```
/// # use overpass_lib::SaniStr;
/// let sani = SaniStr(r#"Dwayne "The Rock" Johnson"#);
/// assert_eq!(&sani.to_string(), r#""Dwayne \"The Rock\" Johnson""#);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SaniStr<'a>(pub &'a str);

impl<'a> From<&'a str> for SaniStr<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

const ESCAPES: LazyLock<HashMap<char, &'static str>> = LazyLock::new(|| {
    HashMap::from([('"', "\\\""), ('\\', "\\\\"), ('\n', "\\n"), ('\t', "\\t")])
});

impl Display for SaniStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "\"")?;
        for c in self.0.chars() {
            if let Some(sub) = ESCAPES.get(&c) {
                write!(f, "{sub}")?;
            } else {
                write!(f, "{c}")?;
            }
        }
        write!(f, "\"")
    }
}

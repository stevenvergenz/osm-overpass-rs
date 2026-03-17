use crate::Namer;
use std::fmt::{Display, Error as FmtError, Formatter, Result as FResult, Write};

/// An error returned when a [Query](crate::Query) cannot produce a valid OverpassQL query string.
#[derive(Debug, Clone)]
pub enum OverpassQLError {
    /// Failed to write to the string.
    Format,
    /// This query contains sets with mutual dependencies, making evaluation impossible.
    CircularReference,
}

impl Display for OverpassQLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::Format => write!(f, "Format"),
            Self::CircularReference => write!(f, "Circular reference"),
        }
    }
}

impl std::error::Error for OverpassQLError {}

impl From<FmtError> for OverpassQLError {
    fn from(_: FmtError) -> Self {
        Self::Format
    }
}
impl Into<FmtError> for OverpassQLError {
    fn into(self) -> FmtError {
        FmtError
    }
}

/// Implementers can be represented as a full or partial OverpassQL query.
pub trait OverpassQL {
    /// Write the OverpassQL representation to the given [Write] object.
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError>;

    /// Write the OverpassQL representation into a String, panicking if the conversion fails.
    fn to_oql(&self) -> String {
        let mut buf = String::new();

        // Bypass format_args!() to avoid write_str with zero-length strs
        OverpassQL::fmt_oql(self, &mut buf)
            .expect("an OverpassQL implementation returned an error unexpectedly");
        buf
    }
}

pub(crate) trait OverpassQLNamed<'a> {
    #[allow(unused_variables)]
    fn fmt_oql_named<'b, 'c>(
        &'b self,
        f: &mut impl Write,
        namer: &mut Namer<'a, 'c>,
    ) -> Result<(), OverpassQLError>
    where
        'b: 'c;
}

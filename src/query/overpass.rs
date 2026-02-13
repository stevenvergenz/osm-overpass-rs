use std::fmt::{Display, Formatter, Write, Result as FResult, Error as FmtError};

#[derive(Debug, Clone)]
pub enum OverpassQLError {
    Format,
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
        FmtError {}
    }
}

pub trait OverpassQL {
    fn fmt_oql(&self, f: &mut impl Write) -> Result<(), OverpassQLError>;

    fn to_oql(&self) -> String {
        let mut buf = String::new();

        // Bypass format_args!() to avoid write_str with zero-length strs
        OverpassQL::fmt_oql(self, &mut buf)
            .expect("an Overpass implementation returned an error unexpectedly");
        buf
    }
}

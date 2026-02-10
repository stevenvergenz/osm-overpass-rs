use std::fmt::{Write, Result as FResult};

pub trait Overpass {
    fn fmt_op(&self, f: &mut impl Write) -> FResult;

    fn to_overpass(&self) -> String {
        let mut buf = String::new();

        // Bypass format_args!() to avoid write_str with zero-length strs
        Overpass::fmt_op(self, &mut buf)
            .expect("an Overpass implementation returned an error unexpectedly");
        buf
    }
}

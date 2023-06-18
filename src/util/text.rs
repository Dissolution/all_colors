use base64::engine::general_purpose::*;
use base64::Engine;
use std::fmt::{Display, Formatter, Result};

pub fn write_base64<T>(value: &T, formatter: &mut Formatter<'_>) -> Result
where
    T: AsRef<[u8]>,
{
    let encoded = STANDARD_NO_PAD.encode(value);
    formatter.write_str(&encoded)
}

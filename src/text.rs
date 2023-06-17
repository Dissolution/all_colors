use base64::engine::general_purpose::*;
use base64::Engine;
use std::fmt::Formatter;

pub fn write_base64<T>(value: &T, formatter: &mut Formatter<'_>) -> std::fmt::Result
where
    T: AsRef<[u8]>,
{
    let mut encoded = STANDARD_NO_PAD.encode(value);
    formatter.write_str(&encoded)
}

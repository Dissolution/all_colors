use crate::util::*;
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AllColorsError {
    #[error("invalid point")]
    InvalidPoint(UPoint),
    #[error("data store disconnected")]
    Parse(#[from] TryFromIntError),
    #[error("missing color")]
    MissingColor(UPoint),
}

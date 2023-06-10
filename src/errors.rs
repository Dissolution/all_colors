use crate::grid::Point;
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AllColorsError {
    #[error("invalid point")]
    InvalidPoint(Point),
    #[error("data store disconnected")]
    Parse(#[from] TryFromIntError),
    #[error("missing color")]
    MissingColor(Point),
}

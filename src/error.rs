pub mod io;

use crate::shared_const::*;
use thiserror::Error;

#[allow(dead_code)]
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Whoops!")]
    SampleVariant,
}

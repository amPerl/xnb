#![warn(missing_docs)]
//! This library provides utilities to read common types from XNB files

mod error;
mod headers;
mod net;
mod object_readers;
mod xnb_reader;

pub use crate::{error::*, headers::*, object_readers::*, xnb_reader::*};

pub use error::Error;
/// Result for operations on this crate's types
pub type XnbResult<T> = Result<T, error::Error>;

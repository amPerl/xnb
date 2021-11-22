use std::io::{Read, Seek};

use crate::ObjectReaderError;

pub mod texture2d;

/// A generic result type for fallible ObjectReader actions
pub type ObjectReaderResult<T> = Result<T, ObjectReaderError>;

/// A trait to represent "Reader" capabilities for a given XNB type
pub trait ObjectReader {
    /// The type of object this reader returns
    type Object;

    /// Returns the descriptor for compatibility checking
    fn desc() -> crate::ObjectReaderDesc;
    /// Attempts to read the object from the reader
    fn read<R: Read + Seek>(reader: &mut R) -> ObjectReaderResult<Self::Object>;
}

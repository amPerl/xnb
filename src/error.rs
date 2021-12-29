use crate::ObjectReaderDesc;

/// Generic errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An internal parser error occurred while reading the header
    #[error("Failed to parse XNB metadata")]
    HeaderParseFailed(binrw::Error),

    /// An internal binrw parser error occurred while reading the readers/shared resources section
    #[error("Failed to parse XNB readers")]
    ReadersParseFailed(binrw::Error),

    /// Reading from the inner reader failed
    #[error("Failed to read from inner reader: {0}")]
    InnerReaderError(#[from] std::io::Error),

    /// Reading compression related data failed (binrw)
    #[error("Reading compression related data failed (binrw): {0}")]
    DecompressBinrwReadFailed(binrw::Error),

    /// Reading compression related data failed
    #[error("Reading compression related data failed: {0}")]
    DecompressReadFailed(binrw::Error),

    /// Decompressing the compressed content failed
    #[error("Failed to decompress content: {0}")]
    DecompressFailed(#[from] lzxd::DecodeFailed),

    /// An XNB feature incompatible with this version of the crate was found while parsing
    #[error("Feature is not yet supported: {0}")]
    FeatureNotSupported(String),

    /// An object type id could not be matched to a reader
    #[error("An object type id could not be matched to a reader: {0}")]
    InvalidTypeId(u8),

    /// An error occurred in an ObjectReader
    #[error("Failed to read object: {0}")]
    ObjectReadFailed(#[from] ObjectReaderError),
}

/// ObjectReader implementation errors
#[derive(Debug, thiserror::Error)]
pub enum ObjectReaderError {
    /// ObjectReader type did not match object type
    #[error("Object descriptor {object_desc:?} did not match reader descriptor {reader_desc:?}")]
    TypeMismatch {
        /// The descriptor for the object being read
        object_desc: ObjectReaderDesc,
        /// The descriptor for the reader expected to be able to read the object
        reader_desc: ObjectReaderDesc,
    },
    /// An internal binrw parser error occurred while reading the object
    #[error("Failed to parse object")]
    ParseFailed(#[from] binrw::Error),
    /// Custom error
    #[error("Custom error: {0}")]
    Custom(#[from] Box<dyn std::error::Error>),
}

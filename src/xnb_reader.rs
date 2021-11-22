use std::io::{Read, Seek};

use binrw::BinReaderExt;

use crate::{
    headers::{ContentHeader, Header},
    object_readers::ObjectReader,
    ObjectReaderDesc, XnbResult,
};

/// A wrapping reader for xnb files
pub struct XnbReader<R: Read + Seek> {
    /// Generic file header
    pub header: Header,
    /// Readers and shared resources
    pub content_header: ContentHeader,
    reader: R,
}

impl<R: Read + Seek> XnbReader<R> {
    /// Parse XNB headers from a reader and return a wrapping reader
    pub fn from_reader(mut reader: R) -> XnbResult<XnbReader<R>> {
        let header: Header = reader.read_le().map_err(crate::Error::HeaderParseFailed)?;

        if header.flags.compressed() {
            return Err(crate::Error::FeatureNotSupported("Compressed data".into()));
        }

        let content_header: ContentHeader =
            reader.read_le().map_err(crate::Error::ReadersParseFailed)?;

        if !content_header.shared_resources.is_empty() {
            return Err(crate::Error::FeatureNotSupported("Shared resources".into()));
        }

        Ok(Self {
            header,
            content_header,
            reader,
        })
    }

    fn next_object_reader_desc(&mut self) -> XnbResult<ObjectReaderDesc> {
        let type_id = self.reader.read_le().unwrap();
        if type_id == 0 {
            return Err(crate::Error::FeatureNotSupported("Null object".into()));
        }

        let reader_index = (type_id - 1) as usize;
        if let Some(object_reader) = self.content_header.readers.get(reader_index) {
            Ok(object_reader.clone())
        } else {
            Err(crate::Error::InvalidTypeId(type_id))
        }
    }

    /// Attempts to read an object with the specified reader.
    ///
    /// Returns an error if the reader's descriptor does not match the object's.
    pub fn read_object<OR: ObjectReader + 'static>(&mut self) -> XnbResult<OR::Object> {
        let object_desc = self.next_object_reader_desc()?;
        let reader_desc = OR::desc();

        if object_desc != reader_desc {
            return Err(crate::ObjectReaderError::TypeMismatch {
                object_desc,
                reader_desc,
            }
            .into());
        }

        Ok(OR::read(&mut self.reader)?)
    }
}

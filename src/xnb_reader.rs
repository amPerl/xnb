use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use binrw::BinReaderExt;

use crate::{
    headers::{ContentHeader, Header},
    object_readers::ObjectReader,
    ObjectReaderDesc, XnbResult,
};

/// A wrapping reader for xnb files
pub struct XnbReader {
    /// Generic file header
    pub header: Header,
    /// Readers and shared resources
    pub content_header: ContentHeader,

    content_reader: std::io::Cursor<Vec<u8>>,
}

impl XnbReader {
    fn read_lzx_chunk<R: Read + Seek>(reader: &mut R) -> XnbResult<Vec<u8>> {
        let flags: u8 = reader
            .read_le()
            .map_err(crate::Error::DecompressBinrwReadFailed)?;

        let _frame_size = if flags == 0xFF {
            // Dynamic frame size, read it in
            reader
                .read_be::<u16>()
                .map_err(crate::Error::DecompressBinrwReadFailed)?
        } else {
            // Fixed frame size, rewind reader by flags size
            let _ = reader.seek(SeekFrom::Current(-1))?;
            0x8000u16
        };

        let block_size = reader
            .read_be::<u16>()
            .map_err(crate::Error::DecompressBinrwReadFailed)?;

        let mut buffer = vec![0u8; block_size as usize];
        reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    /// Parse XNB headers from a reader and return a wrapping reader
    pub fn from_reader<R: Read + Seek>(mut reader: R) -> XnbResult<XnbReader> {
        let header: Header = reader.read_le().map_err(crate::Error::HeaderParseFailed)?;

        let content_buffer = if header.flags.compressed() {
            let mut content_buffer =
                Vec::with_capacity(header.decompressed_size.unwrap_or(0) as usize);

            if header.flags.compressed_lz4() {
                return Err(crate::Error::FeatureNotSupported("LZ4 Compression".into()));
            }

            let mut lzxd = lzxd::Lzxd::new(lzxd::WindowSize::KB64);

            loop {
                if content_buffer.len() == content_buffer.capacity() {
                    break;
                }

                let compressed_chunk = Self::read_lzx_chunk(&mut reader)?;
                let decompressed_chunk = lzxd.decompress_next(&compressed_chunk)?;
                content_buffer.write_all(decompressed_chunk)?;
            }

            content_buffer
        } else {
            let mut content_buffer = vec![0u8; header.file_size as usize - 10];

            reader.read_exact(&mut content_buffer)?;

            content_buffer
        };

        let mut content_reader = Cursor::new(content_buffer);

        let content_header: ContentHeader = content_reader
            .read_le()
            .map_err(crate::Error::ReadersParseFailed)?;

        if !content_header.shared_resources.is_empty() {
            return Err(crate::Error::FeatureNotSupported("Shared resources".into()));
        }

        Ok(Self {
            header,
            content_header,
            content_reader,
        })
    }

    fn next_object_reader_desc(&mut self) -> XnbResult<ObjectReaderDesc> {
        let type_id = self.content_reader.read_le().unwrap();
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

        let reader_name_matches = object_desc.reader == reader_desc.reader
            || object_desc.reader.starts_with(&reader_desc.reader);

        if !reader_name_matches || object_desc.version != reader_desc.version {
            return Err(crate::ObjectReaderError::TypeMismatch {
                object_desc,
                reader_desc,
            }
            .into());
        }

        Ok(OR::read(&mut self.content_reader)?)
    }
}

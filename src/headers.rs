use binrw::BinRead;

/// XNB header
/// In the beginning of the file
/// Always plain (uncompressed)
#[derive(BinRead, Debug, PartialEq)]
#[br(magic = b"XNB")]
pub struct Header {
    /// The target platform this XNB was built for
    pub target: TargetPlatform,
    /// The version of the XNB file format
    pub format_version: FormatVersion,
    /// Compression and quality flags
    pub flags: Flags,
    /// File size on disk
    pub file_size: u32,
    /// File size when decompressed
    #[br(if(flags.compressed()))]
    pub decompressed_size: Option<u32>,
}

/// XNB content header, parsed after the header.
/// Potentially compressed, see header's flags or decompressed_size
#[derive(BinRead, Debug, PartialEq)]
#[br(assert(shared_resource_count == 0, "Shared resources are not yet supported"))]
pub struct ContentHeader {
    #[br(parse_with = crate::net::parse_7bit_encoded_int)]
    reader_count: usize,
    /// Reader definitions for parsing objects
    #[br(count = reader_count)]
    pub readers: Vec<ObjectReaderDesc>,

    #[br(parse_with = crate::net::parse_7bit_encoded_int)]
    shared_resource_count: usize,
    /// Shared resources used to flatten nested objects
    #[br(count = shared_resource_count)]
    pub shared_resources: Vec<()>,
}

/// All known target platforms
#[derive(BinRead, Debug, PartialEq)]
#[repr(u8)]
pub enum TargetPlatform {
    /// Windows
    #[br(magic = b'w')]
    Windows,
    /// Windows Phone 7 (?)
    #[br(magic = b'm')]
    WindowsPhone,
    /// Xbox 360 (?)
    #[br(magic = b'x')]
    Xbox,
    /// Generic Android device
    #[br(magic = b'a')]
    Android,
    /// Generic iOS device
    #[br(magic = b'i')]
    Ios,
}

/// All known XNB file format versions
#[derive(BinRead, Debug, PartialEq)]
#[repr(u8)]
pub enum FormatVersion {
    /// XNA 3.0 (version 3)
    #[br(magic = 3u8)]
    Xna3,
    /// XNA 3.1 (version 4)
    #[br(magic = 4u8)]
    Xna3_1,
    /// XNA 4.0 (version 5)
    #[br(magic = 5u8)]
    Xna4,
}

/// Flags describing whether the content is Reach/HiDef and/or compressed
#[derive(BinRead, Debug, PartialEq)]
#[repr(u8)]
pub enum Flags {
    /// Uncompressed (0x80 is 0), Reach (0x01 is 0)
    #[br(magic = 0x00u8)]
    UncompressedReach,
    /// Compressed (0x80 is 1), Reach (0x01 is 0)
    #[br(magic = 0x80u8)]
    CompressedReach,
    /// Uncompressed (0x80 is 0), HiDef (0x01 is 1)
    #[br(magic = 0x01u8)]
    UncompressedHiDef,
    /// Compressed (0x80 is 1), HiDef (0x01 is 1)
    #[br(magic = 0x81u8)]
    CompressedHiDef,
}

impl Flags {
    /// Returns whether or not the flags represent compressed content
    pub fn compressed(&self) -> bool {
        match self {
            Flags::CompressedReach | Flags::CompressedHiDef => true,
            Flags::UncompressedReach | Flags::UncompressedHiDef => false,
        }
    }
}

/// XNB header definition for a reader type
#[derive(BinRead, Debug, PartialEq, Clone)]
pub struct ObjectReaderDesc {
    /// The fully qualified type of the reader
    #[br(parse_with = crate::net::parse_string)]
    pub reader: String,
    /// Version of the reader for the given type
    pub version: u32,
}

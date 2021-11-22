//! Reader for `Microsoft.Xna.Framework.Graphics.Texture2D`.
//! Aiming to behave like `Microsoft.Xna.Framework.Graphics.Texture2DReader`

use std::io::{Read, Seek};

use binrw::{BinRead, BinReaderExt};

use super::{ObjectReader, ObjectReaderResult};

/// Microsoft.Xna.Framework.Graphics.Texture2D
#[derive(BinRead, Debug, PartialEq)]
pub struct Texture2D {
    /// Surface format
    pub surface_format: SurfaceFormat,
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Texture mip count
    pub mip_count: u32,
    /// Texture mips
    #[br(count = mip_count)]
    pub mips: Vec<Mip>,
}

impl ObjectReader for Texture2D {
    type Object = Texture2D;

    fn desc() -> crate::ObjectReaderDesc {
        crate::ObjectReaderDesc {
            reader: "Microsoft.Xna.Framework.Content.Texture2DReader".into(),
            version: 0,
        }
    }

    fn read<R: Read + Seek>(reader: &mut R) -> ObjectReaderResult<Texture2D> {
        let tex: Texture2D = reader.read_le()?;
        Ok(tex)
    }
}

/// A mip in the surface format described in the parent Texture2D
#[derive(BinRead, Debug, PartialEq)]
pub struct Mip {
    /// Data size in bytes
    pub data_size: u32,
    /// Data buffer, data_size bytes
    #[br(count = data_size)]
    pub image_data: Vec<u8>,
}

/// Surface format of a Mip in Texture2D
#[derive(BinRead, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum SurfaceFormat {
    #[br(magic = 0u32)]
    Color,
    #[br(magic = 1u32)]
    Bgr565,
    #[br(magic = 2u32)]
    Bgra5551,
    #[br(magic = 3u32)]
    Bgra4444,
    #[br(magic = 4u32)]
    Dxt1,
    #[br(magic = 5u32)]
    Dxt3,
    #[br(magic = 6u32)]
    Dxt5,
    #[br(magic = 7u32)]
    NormalizedByte2,
    #[br(magic = 8u32)]
    NormalizedByte4,
    #[br(magic = 9u32)]
    Rgba1010102,
    #[br(magic = 10u32)]
    Rg32,
    #[br(magic = 11u32)]
    Rgba64,
    #[br(magic = 12u32)]
    Alpha8,
    #[br(magic = 13u32)]
    Single,
    #[br(magic = 14u32)]
    Vector2,
    #[br(magic = 15u32)]
    Vector4,
    #[br(magic = 16u32)]
    HalfSingle,
    #[br(magic = 17u32)]
    HalfVector2,
    #[br(magic = 18u32)]
    HalfVector4,
    #[br(magic = 19u32)]
    HdrBlendable,
}

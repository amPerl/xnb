use binrw::{BinResult, ReadOptions};
use std::io::{Read, Seek};

pub fn parse_string<R: Read + Seek>(
    reader: &mut R,
    options: &ReadOptions,
    args: (),
) -> BinResult<String> {
    let length = super::varint::parse_7bit_encoded_int(reader, options, args)?;

    let mut str_buf = vec![0u8; length];
    reader.read_exact(&mut str_buf)?;

    String::from_utf8(str_buf).map_err(|err| binrw::Error::Custom {
        pos: reader.stream_position().unwrap(),
        err: Box::new(err),
    })
}

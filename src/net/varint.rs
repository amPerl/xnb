use binrw::{BinReaderExt, BinResult, ReadOptions};
use std::io::{Read, Seek};

pub fn parse_7bit_encoded_int<R: Read + Seek>(
    reader: &mut R,
    _options: &ReadOptions,
    _args: (),
) -> BinResult<usize> {
    let mut result = 0usize;
    let mut bits_read = 0;
    loop {
        let value: u8 = reader.read_le()?;

        result |= ((value & 0x7f) as usize) << bits_read;
        bits_read += 7;

        if value & 0x80 == 0 {
            break;
        }
    }
    Ok(result)
}

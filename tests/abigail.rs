use std::io::Cursor;

use xnb::{Error, XnbReader};

#[test]
pub fn abigail() {
    let abigail_bytes = include_bytes!("./Abigail.xnb");

    let mut cursor = Cursor::new(abigail_bytes);

    let xnb_result = XnbReader::from_reader(&mut cursor);

    assert!(matches!(xnb_result, Err(Error::FeatureNotSupported(_))));
}

use std::io::Cursor;

use xnb::{TargetPlatform, XnbReader, XnbResult};

#[test]
pub fn abigail() -> XnbResult<()> {
    let abigail_bytes = include_bytes!("./Abigail.xnb");

    let mut cursor = Cursor::new(abigail_bytes);

    let xnb_reader = XnbReader::from_reader(&mut cursor)?;

    assert_eq!(xnb_reader.header.target, TargetPlatform::Windows);

    assert_eq!(xnb_reader.content_header.readers.len(), 1);
    assert_eq!(
        xnb_reader.content_header.readers[0].reader,
        "Microsoft.Xna.Framework.Content.Texture2DReader, Microsoft.Xna.Framework.Graphics, Version=4.0.0.0, Culture=neutral, PublicKeyToken=842cf8be1de50553".to_string()
    );

    Ok(())
}

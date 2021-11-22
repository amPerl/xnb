use std::io::Cursor;

use xnb::{texture2d::Texture2D, TargetPlatform, XnbReader, XnbResult};

#[test]
pub fn arrows() -> XnbResult<()> {
    let arrows_bytes = include_bytes!("./arrows.xnb");

    let mut cursor = Cursor::new(arrows_bytes);

    let mut xnb_reader = XnbReader::from_reader(&mut cursor)?;

    assert_eq!(xnb_reader.header.target, TargetPlatform::Windows);

    assert_eq!(xnb_reader.content_header.readers.len(), 1);
    assert_eq!(
        xnb_reader.content_header.readers[0].reader,
        "Microsoft.Xna.Framework.Content.Texture2DReader".to_string()
    );

    let tex = xnb_reader.read_object::<Texture2D>()?;
    assert_eq!(tex.width, 16);
    assert_eq!(tex.height, 7);

    Ok(())
}

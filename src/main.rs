use ico::{IconDir, IconImage};
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    // Create a new .ico directory.
    let mut icon_dir: IconDir = IconDir::new(ico::ResourceType::Icon);

    let file: File = File::open("input/image.png")?;
    let image: IconImage = IconImage::read_png(file)?;
    icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
    let file: File = File::create("output.ico").unwrap();
    icon_dir.write(file)
}

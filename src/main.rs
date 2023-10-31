use std::io::Result;
use std::fs;
use std::fs::File;
use ico::{IconDir, IconImage};

fn main() -> Result<()> {
    // Create a new .ico directory.
    let mut icon_dir: IconDir = IconDir::new(ico::ResourceType::Icon);

    let input_dir = "src/input";
    let output_dir = "src/output";

    fs::create_dir_all(output_dir)?;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;

        if let Some(file_path) = entry.path().to_str() {
            if file_path.ends_with(".png") {
                // This file is a PNG file
                let file = File::open(file_path)?;
                let image: IconImage = IconImage::read_png(file)?;

                // Add this image to the ICO directory
                icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
            }
        }
    }
    // Construct the output file path
    let output_file_path = format!("{}/output.ico", output_dir);

    let output_file = File::create(output_file_path)?;
    icon_dir.write(output_file)
}

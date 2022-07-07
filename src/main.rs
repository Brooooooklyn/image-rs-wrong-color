use std::error;
use std::fs;

use image::load_from_memory;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let raw_png = fs::read("hulu.png")?;
    let dynamic_image = load_from_memory(&raw_png)?;
    dynamic_image.save_with_format("hulu-image.png", image::ImageFormat::Png)?;
    Ok(())
}

use image::{ImageError, ImageReader};

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("photo.jpeg")?.decode()?;
    println!("Dimensions: {} x {}", img.width(), img.height());
    Ok(())
}

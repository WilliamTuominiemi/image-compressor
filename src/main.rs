use image::{ImageError, ImageReader};

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("photo.jpeg")?.decode()?;
    let rgb = img.into_rgb16();

    println!("Dimensions: {} x {}", rgb.width(), rgb.height());

    println!("{:?}", rgb);

    Ok(())
}

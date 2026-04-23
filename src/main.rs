use image::{GenericImageView, ImageError, ImageReader};

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("photo.jpeg")?.decode()?;
    println!("Dimensions: {} x {}", img.width(), img.height());

    for (x, y, pixel) in img.pixels() {
        let rgba = pixel.0;
        println!(
            "Pixel at ({}, {}): Red={}, Green={}, Blue={}, Alpha={}",
            x, y, rgba[0], rgba[1], rgba[2], rgba[3]
        );
    }

    Ok(())
}

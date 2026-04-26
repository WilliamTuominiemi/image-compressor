use image::{ImageError, ImageReader};

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("photo.jpeg")?.decode()?;
    let mut rgb = img.into_rgb8();

    for (x, y, pixel) in rgb.enumerate_pixels_mut() {
        pixel.0[2] = 0;
        pixel.0[0] = pixel.0[0].saturating_add(50);
    }

    rgb.save("altered_photo.jpg")?;

    Ok(())
}

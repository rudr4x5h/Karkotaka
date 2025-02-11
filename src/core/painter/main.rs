use anyhow::Error;
use cosmic_text::Color;
use image::{Rgb, RgbImage};

use image_utils::{read_image, resize_fill, save_image_buffer, Format};

fn main() -> Result<(), Error> {
    // let img = read_image("assets/sample-1.jpg")?;
    let img = read_image("assets/sample-3.webp")?;
    let resized = resize_fill(&img, 1080, 1350)?;

    // Create a new image with a solid black background
    let mut background_image = RgbImage::new(1080, 1350);
    for pixel in background_image.pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }

    for (x, y, pixel) in resized.to_rgb8().enumerate_pixels() {
        background_image.put_pixel(x, y, *pixel);
    }

    // save_image(&background_image, "assets/OUTPUT")?;
    Ok(())
}

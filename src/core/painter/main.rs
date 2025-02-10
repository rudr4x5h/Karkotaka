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

    let primary_headline = "The al-Jaili Refinery was attacked on January 25, 2025, causing toxic smoke that threatens health.";
    let secondary_headline = "Sudan's military accuses the Rapid Support Forces (RSF) of intentionally burning the al-Jaili Refinery, while RSF claims the military used barrel bombs without evidence.";
    // bounds = (x, y, width, height) = for text container.
    let bounds_primary = (54.0, 900.0, 972.0, 450.0);
    let bounds_secondary = (54.0, 900.0 + (70.0 * 3.0 + 21.0), 972.0, 200.0);

    // Specify words to highlight and highlight color
    let highlight_words_sample = &["al-Jaili Refinery", "toxic smoke", "health"];
    let highlight_words = highlight_words_sample
        .join(" ")
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    font::draw_text_with_background(
        primary_headline,
        secondary_headline,
        &mut background_image,
        Color::rgb(242, 222, 160),
        bounds_primary,
        bounds_secondary,
        highlight_words.as_slice(),
        Color::rgb(217, 41, 41), // Dark red highlight color
        60.0,
        70.0,
    )?;

    save_image_buffer(&background_image, "assets/OUTPUT", Format::JPG)?;

    // save_image(&background_image, "assets/OUTPUT")?;
    Ok(())
}

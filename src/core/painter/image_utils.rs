use anyhow::Error;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader, RgbImage};
use uuid::Uuid;

/// Resize the image to fill the given width and height, keeping the aspect ratio.
pub fn resize_fill(img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage, Error> {
    let (w, h) = img.dimensions();
    if w == width && h == height {
        return Ok(img.clone());
    }
    let resized = img.resize_to_fill(width, height, FilterType::Lanczos3);
    println!("resized to: {:?}", resized.dimensions());
    Ok(resized)
}

/// Save the image to the given directory.
pub fn save_image(img: &DynamicImage, dir: &str) -> Result<(), Error> {
    let uid = Uuid::new_v4();
    let file_name = format!("{}.jpg", uid);
    let path = format!("{}/{}", dir, file_name);
    println!("saving to: {}", path);
    img.save(path)?;
    Ok(())
}

pub enum Format {
    PNG,
    JPG,
}

pub fn save_image_buffer(img: &RgbImage, output_dir: &str, format: Format) -> Result<(), Error> {
    let uid = Uuid::new_v4();
    let file_name = match format {
        Format::PNG => format!("{}.png", uid),
        Format::JPG => format!("{}.jpg", uid),
    };

    let path = format!("{}/{}", output_dir, file_name);
    println!("saving to: {}", path);
    img.save(path)?;
    Ok(())
}

/// Read the image from the given path.
pub fn read_image(path: &str) -> Result<DynamicImage, Error> {
    let img = ImageReader::open(path)?.decode()?;
    println!("read image from: {}", path);
    Ok(img)
}

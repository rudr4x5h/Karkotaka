use crate::core::{search::FoundStory, secondary::image::Image};

pub fn gen_image(found_story: FoundStory) -> Image {
    // save the image on fs/object storage,
    // and return the publicly accessible URI.
    let image_uri = format!("testing.png");
    Image::new(image_uri)
}

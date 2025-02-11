use crate::core::{
    painter::{
        self,
        build::{Builder, Canvas, CanvasBuilder},
    },
    search::FoundStory,
    secondary::image::Image,
};

use crate::core::painter::paint;

pub fn gen_image(found_story: FoundStory) -> Image {
    let headshot = found_story.clone().get_headshot();
    let synopsis = found_story.clone().get_synopsis();
    let headline = found_story.clone().get_headline();

    let mut canvas_builder = CanvasBuilder::new();
    let canvas = canvas_builder.build();

    let painted_img_buffer = paint::draw_text_with_background(
        canvas,
        primary_font_file,
        secondary_font_file,
        font_family,
    )
    .expect("Error - Cannot paint image.");

    let image_uri =
        format!("https://cdn.pixabay.com/photo/2022/08/26/13/15/man-7412527_960_720.png");
    Image::new(image_uri)
}

pub fn get_llm_image(gen_prompt: String) -> Image {
    // call the llm for image
    // get the response, parse uri, download the image
    // save it to storage, get the path
    // crate an image, return
    todo!()
}

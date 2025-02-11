use cosmic_text::Color;

use crate::core::{
    painter::{
        build::{Builder, Canvas, CanvasBuilder},
        image_utils::{save_image_buffer, Format},
    },
    primary::{headline::Headline, headshot::Headshot, synopsis::Synopsis},
    search::FoundStory,
    secondary::image::Image,
    utils::error::AppError,
};

use crate::core::painter::paint;

pub fn _get_canvas(
    headshot: Headshot,
    synopsis: Synopsis,
    headline: Headline,
) -> Result<Canvas, AppError> {
    let mut canvas_builder = CanvasBuilder::new();
    canvas_builder.set_headshot(headshot);
    canvas_builder.set_synopsis(synopsis);
    canvas_builder.set_headline(headline);
    canvas_builder.set_style(
        60.0,
        70.0,
        2.0,
        Color::rgb(255, 255, 0),
        Color::rgb(255, 0, 0),
    );
    let (x, y, width, height) = (54.0, 900.0, 972.0, 450.0);
    canvas_builder.set_bounds_p(x, y, width, height);
    canvas_builder.set_bounds_s(x, y + (70.0 * 3.0), width, height / 2.0);
    canvas_builder.set_width(1080);
    canvas_builder.set_height(1350);
    Ok(canvas_builder.build())
}

pub fn gen_image(found_story: FoundStory) -> Result<Image, AppError> {
    let headshot = found_story.clone().get_headshot();
    let synopsis = found_story.clone().get_synopsis();
    let headline = found_story.clone().get_headline();

    let font_family = "Outfit";
    let primary_font = format!("assets/{}.ttf", font_family);
    dbg!(primary_font.clone());

    let canvas = _get_canvas(headshot, synopsis, headline)?;
    let painted_img_buffer = paint::draw_text_with_background(
        canvas,
        primary_font.as_str(),
        primary_font.as_str(),
        font_family,
    )
    .expect("Error - Cannot paint image.");

    let generated_image_bucket = "/home/rudr4x5h/FOUNDRY/Karkotaka/assets/OUTPUT_BASKET";
    let save_path = save_image_buffer(&painted_img_buffer, generated_image_bucket, Format::JPG)?;
    Ok(Image::new(save_path))
}

pub fn get_llm_image(gen_prompt: String, found_story: FoundStory) -> Image {
    todo!()
}

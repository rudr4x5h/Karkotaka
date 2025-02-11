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

pub fn gen_final_image(found_story: FoundStory) -> Result<Image, AppError> {
    let headshot = found_story.clone().get_headshot();
    let synopsis = found_story.clone().get_synopsis();
    let headline = found_story.clone().get_headline();

    let font_family = "Outfit";
    let primary_font = format!(
        "/home/rudr4x5h/FOUNDRY/Karkotaka/assets/{}.ttf",
        font_family
    );
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

pub fn gen_llm_image(gen_prompt: String, found_story: FoundStory) -> Image {
    // Get Ollama server URL from environment variable or use default localhost
    let base_url = std::env::var("OLLAMA_URL").unwrap_or("http://127.0.0.1:11434".into());

    // Initialize and configure the LLM client
    let llm = LLMBuilder::new()
        .backend(LLMBackend::Ollama) // Use Ollama as the LLM backend
        .base_url(base_url) // Set the Ollama server URL
        .model("deepseek-r1:1.5b")
        .max_tokens(1000) // Set maximum response length
        .temperature(0.7) // Control response randomness (0.0-1.0)
        .stream(false) // Disable streaming responses
        .build()
        .expect("Failed to build LLM (Ollama)");

    // Prepare conversation history with example messages
    let messages = vec![
        ChatMessage {
            role: ChatRole::User,
            content: "Hello, how do I run a local LLM in Rust?".into(),
        },
        ChatMessage {
            role: ChatRole::Assistant,
            content: "One way is to use Ollama with a local model!".into(),
        },
        ChatMessage {
            role: ChatRole::User,
            content: "Tell me more about that".into(),
        },
    ];

    // Send chat request and handle the response
    match llm.chat(&messages).await {
        Ok(text) => println!("Ollama chat response:\n{}", text),
        Err(e) => eprintln!("Chat error: {}", e),
    }

    Ok(())
}

use cosmic_text::Color;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, ChatRole},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

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

pub async fn gen_llm_image(gen_prompt: String, found_story: FoundStory) {}

pub async fn gen_llm_synopsis(found_story: FoundStory) -> Option<GeneratedSynopsis> {
    let base_url = std::env::var("OLLAMA_URL").unwrap_or("http://127.0.0.1:11434".into());

    let llm = LLMBuilder::new()
        .backend(LLMBackend::Ollama) // Use Ollama as the LLM backend
        .base_url(base_url) // Set the Ollama server URL
        .model("granite3.1-dense:8b")
        .max_tokens(112) // Set maximum response length
        .temperature(0.7) // Control response randomness (0.0-1.0)
        .stream(false) // Disable streaming responses
        .build()
        .expect("Failed to build LLM (Ollama)");

    let story = found_story.to_story().await;
    let body: Vec<String> = story
        .get_body()
        .get_paragraphs()
        .iter()
        .map(|p| p.get_content().to_owned())
        .collect();
    let combined_story = format!(
        "{}. {}",
        story.get_headline().get_content(),
        body.join(". ")
    );

    dbg!(combined_story.clone());

    let messages = vec![
        ChatMessage {
            role: ChatRole::User,
            content: "You are an expert in content summarization with focus on keeping the summarized content precise and information loaded. Summarize the article given by user into three complete and independent themes.".into(),
        },
        ChatMessage {
            role: ChatRole::Assistant,
            content: "Okay, I will summarize the article given by user into 3 distinct but complete themes presenting different viewpoints or aspect of the article, avoiding any unnecessary fillers, and including relevant names, places, dates of other significant data like percentages.".into(),
        },
        ChatMessage {
            role: ChatRole::User,
            content: "Please summarize this article into 3 distinct themes, within 70 words. Do not use numbering. Write short but complete sentences.".into(),
        },
        ChatMessage {
            role: ChatRole::User,
            content: format!("article: {}", combined_story)
        },
        ChatMessage {
            role: ChatRole::User,
            content: "return response in a valid JSON, like {'synopses': ['theme1', 'theme2', 'theme3']}".into()
        }
    ];

    let mut response: Option<String> = None;
    match llm.chat(&messages).await {
        Ok(text) => {
            response = {
                let re = Regex::new(r"<think>[\w\W]*</think>").expect("Error parsing regex");
                let trimmed_response = re.replace_all(text.as_str(), "").to_string();
                Some(trimmed_response)
            }
        }
        Err(e) => eprintln!("LLM Syn Error: {}", e),
    }

    let clean_response = clean_json(response.unwrap()).unwrap();
    let response: GeneratedSynopsis = serde_json::from_str(clean_response.as_str()).unwrap();
    Some(response)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedSynopsis {
    synopses: Vec<String>,
}

pub fn clean_json(json_str: String) -> Result<String, AppError> {
    let cleaned = json_str.replace(r"\", "");
    Ok(cleaned)
}

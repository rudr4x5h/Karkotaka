use axum::debug_handler;
use axum::extract::{Json, Path};
use axum::response::IntoResponse;
use uuid::Uuid;

use super::error::AppError;
use super::persistence::DB;
use crate::core::primary::body::Body;
use crate::core::primary::headline::Headline;
use crate::core::primary::headshot::Headshot;
use crate::core::primary::story::{Story, StoryWithId, STORY_DB};
use crate::core::primary::synopsis::Synopsis;
use crate::core::secondary::image::Image;
use crate::core::secondary::misc::{Kind, Record};
use crate::core::secondary::paragraph::Paragraph;
use crate::core::secondary::report::Report;

pub async fn root() -> impl IntoResponse {
    "Namaskaram"
}

#[debug_handler]
pub async fn create_story(Json(content): Json<String>) -> Result<Json<StoryWithId>, AppError> {
    let kind = Kind::OG;
    let headline = Headline::new(content, kind);
    let story = Story::new(headline);

    let record: Option<StoryWithId> = DB
        // .create((STORY_DB, story.clone().get_id().to_string()))
        .create(STORY_DB)
        .content(story.clone())
        .await?;

    Ok(Json(record.unwrap()))
}

#[debug_handler]
pub async fn add_headshot(
    Path(story_id): Path<Uuid>,
    Json(uri): Json<String>,
) -> Result<Json<Story>, AppError> {
    let kind = Kind::OG;
    let image = Image::new(uri);
    let headshot = Headshot::new(kind, image);

    let state = DB.clone();
    let story = state
        .update((STORY_DB, story_id.to_string()))
        .content(headshot)
        .await?;
    Ok(Json(story.unwrap()))
}

pub async fn add_synopsis(
    Path(story_id): Path<Uuid>,
    Json(content): Json<String>,
) -> Result<Json<Story>, AppError> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind.clone());

    let mut synopsis = Synopsis::new(kind.clone());
    synopsis.add_paragraph(paragraph);

    let state = DB.clone();
    let story = state
        .update((STORY_DB, story_id.to_string()))
        .content(synopsis)
        .await?;
    Ok(Json(story.unwrap()))
}

pub async fn add_body(
    Path(story_id): Path<Uuid>,
    Json(paragraphs): Json<Vec<String>>,
) -> Result<Json<Story>, AppError> {
    let paras = _get_para_from_strings(paragraphs.clone()).await;
    let body = Body::from_paras(paras);

    let state = DB.clone();
    let story = state
        .update((STORY_DB, story_id.to_string()))
        .content(body)
        .await?;
    Ok(Json(story.unwrap()))
}

async fn _get_para_from_strings(paragraphs: Vec<String>) -> Vec<Paragraph> {
    let paras = paragraphs
        .iter()
        .map(|content| {
            let kind = Kind::OG;
            Paragraph::new(content.clone(), kind)
        })
        .collect();

    paras
}

pub async fn add_paragraph(
    Path(story_id): Path<Uuid>,
    Json(content): Json<String>,
) -> Result<Json<Story>, AppError> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind);

    let state = DB.clone();
    let mut story: Story = state
        .select((STORY_DB, story_id.to_string()))
        .await?
        .unwrap();

    let body = story.get_body();
    dbg!(body.get_paragraphs());

    Ok(Json(story))
}

pub async fn report_story(Path(story_id): Path<Uuid>) -> Result<Json<Report>, AppError> {
    let state = DB.clone();
    let story: Story = state
        .select((STORY_DB, story_id.to_string()))
        .await?
        .unwrap();

    let report = Report::new(story);
    Ok(Json(report))
}

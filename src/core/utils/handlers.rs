use std::collections::HashMap;

use anyhow::anyhow;
use axum::debug_handler;
use axum::extract::{Json, Path, Query};
use axum::response::IntoResponse;
use surrealdb::opt::PatchOp;

use super::error::AppError;
use super::persistence::DB;
use crate::core::plug;
use crate::core::plug::llm::{gen_llm_synopsis, GeneratedSynopsis};
use crate::core::primary::body::Body;
use crate::core::primary::headline::Headline;
use crate::core::primary::headshot::Headshot;
use crate::core::primary::story::{Story, StoryWithId, STORY_DB};
use crate::core::primary::synopsis::Synopsis;
use crate::core::search::{FoundStory, Search, SearchResults};
use crate::core::secondary::image::Image;
use crate::core::secondary::misc::{self, GenRequest, GenRequestResponse, Kind};
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

    let record: Option<StoryWithId> = DB.create(STORY_DB).content(story.clone()).await?;
    Ok(Json(record.unwrap()))
}

#[debug_handler]
pub async fn add_headshot(
    Path(story_id): Path<String>,
    Json(uri): Json<String>,
) -> Result<Json<Story>, AppError> {
    let kind = Kind::OG;
    let images = vec![Image::new(uri)];
    let headshot = Headshot::new(kind, images);

    let story = misc::str_to_recordid((STORY_DB.to_string(), story_id));
    let record = DB
        .update(story)
        .patch(PatchOp::replace("/headshot", headshot))
        .await?;
    Ok(Json(record.unwrap()))
}

pub async fn add_synopsis(
    Path(story_id): Path<String>,
    Json(content): Json<Vec<String>>,
) -> Result<Json<Story>, AppError> {
    let kind = Kind::OG;
    let story = misc::str_to_recordid((STORY_DB.to_string(), story_id.to_string()));

    let mut synopsis = Synopsis::new(kind.clone());
    for para_str in content {
        let para = Paragraph::new(para_str, kind.clone());
        synopsis.add_paragraph(para);
    }

    // tokio::spawn(move || async {
    //     let gen_syn = gen_llm_synopsis(found_story);
    // });

    let record = DB
        .update(story)
        .patch(PatchOp::replace("/synopsis", synopsis))
        .await?;
    Ok(Json(record.unwrap()))
}

pub async fn add_body(
    Path(story_id): Path<String>,
    Json(paragraphs): Json<Vec<String>>,
) -> Result<Json<Story>, AppError> {
    let paras = _get_para_from_strings(paragraphs.clone()).await;
    let body = Body::from_paras(paras);

    let story = misc::str_to_recordid((STORY_DB.to_string(), story_id.to_string()));
    let record = DB
        .update(story)
        .patch(PatchOp::replace("/body", body))
        .await?;
    Ok(Json(record.unwrap()))
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
    Path(story_id): Path<String>,
    Json(content): Json<String>,
) -> Result<Json<StoryWithId>, AppError> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind);

    let story_with_id = misc::str_to_recordid((STORY_DB.to_string(), story_id.to_string()));
    let mut record: Story = DB.select(story_with_id.clone()).await?.unwrap();
    record.get_body_mut().add_paragraph(paragraph);

    let record: Option<StoryWithId> = DB.update(story_with_id.clone()).merge(record).await?;
    Ok(Json(record.unwrap()))
}

pub async fn report_story(Path(story_id): Path<String>) -> Result<Json<Report>, AppError> {
    let story: Story = DB.select((STORY_DB, story_id.to_string())).await?.unwrap();

    let report = Report::new(story);
    Ok(Json(report))
}

pub async fn search_stories(
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<SearchResults>, AppError> {
    let query = query.get("query").unwrap().to_owned();
    let search = Search::new(query);
    let results = search.execute().await?;

    Ok(Json(results))
}

/// TODO - return instantaneously a generation ID, using which
/// the user can track whether the image has been generated.
/// and then can query DB for content.
pub async fn request_generation(
    Json(request): Json<GenRequest>,
) -> Result<Json<GenRequestResponse>, AppError> {
    // ) -> Result<Json<String>, AppError> {
    // first: n articles n image.
    let mut images = HashMap::new();

    for story in request.clone().get_stories() {
        let found_story = story.clone();
        let story_id = found_story.clone().get_id().key().to_string();
        let final_image = plug::llm::gen_final_image(found_story.clone())?;
        let mut headshot = found_story.clone().get_headshot();
        headshot.add_image(final_image.clone());

        let story_with_id = misc::str_to_recordid((STORY_DB.to_string(), story_id.clone()));
        let _: Option<Story> = DB
            .update(story_with_id)
            .patch(PatchOp::replace("/headshot", headshot))
            .await?;

        images.insert(story_id, final_image);
    }
    let img_ct = request.clone().get_image_count();

    let response = GenRequestResponse::new(img_ct, images);
    Ok(Json(response))
}

pub async fn gen_syn(Json(content): Json<FoundStory>) -> Result<Json<GeneratedSynopsis>, AppError> {
    let response = gen_llm_synopsis(content).await.unwrap();
    Ok(Json(response))
}

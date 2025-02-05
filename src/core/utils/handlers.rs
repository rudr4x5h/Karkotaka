use anyhow::Error;
use axum::extract::{Json, Query, State};
use axum::response::IntoResponse;
use std::sync::{Arc, Mutex};
use ulid::Ulid;

use crate::core::primary::body::Body;
use crate::core::primary::headline::Headline;
use crate::core::primary::headshot::Headshot;
use crate::core::primary::story::Story;
use crate::core::primary::synopsis::Synopsis;
use crate::core::secondary::image::Image;
use crate::core::secondary::misc::Kind;
use crate::core::secondary::paragraph::Paragraph;
use crate::core::secondary::report::Report;

use super::persistence::{Persist, PersistInMemory};

pub async fn root() -> impl IntoResponse {
    "Namaskaram"
}

pub async fn create_story(
    Json(content): Json<String>,
    State(state): State<Arc<Mutex<PersistInMemory>>>,
) -> Result<Json<Story>, Error> {
    let kind = Kind::OG;
    let headline = Headline::new(content, kind);
    let story = Story::new(headline);

    let mut state = state.lock().unwrap();
    let story_id = state.save(story.clone());
    match story_id {
        Ok(_) => Ok(Json(story.clone())),
        Err(e) => Err(e),
    }
}

pub async fn add_headshot(
    Json(uri): Json<String>,
    Query(story_id): Query<Ulid>,
    State(state): State<Arc<Mutex<PersistInMemory>>>,
) -> Result<Json<Story>, Error> {
    let kind = Kind::OG;
    let image = Image::new(uri);
    let headshot = Headshot::new(kind, image);

    let mut state = state.lock().unwrap();
    let story = state.load(story_id).unwrap();
    story.set_headshot(headshot);
    Ok(Json(story.clone()))
}

pub async fn add_synopsis(
    Json(content): Json<String>,
    Query(story_id): Query<Ulid>,
    State(state): State<Arc<Mutex<PersistInMemory>>>,
) -> Result<Json<Story>, Error> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind.clone());

    let mut synopsis = Synopsis::new(kind.clone());
    synopsis.add_paragraph(paragraph);

    let mut state = state.lock().unwrap();
    let story = state.load(story_id).unwrap();
    story.set_synopsis(synopsis.clone());

    Ok(Json(story.clone()))
}

pub async fn add_body(
    Json(paragraphs): Json<Vec<Paragraph>>,
    Query(story_id): Query<Ulid>,
    State(state): State<Arc<Mutex<PersistInMemory>>>,
) -> Result<Json<Story>, Error> {
    let body = Body::from_paras(paragraphs);

    let mut state = state.lock().unwrap();
    let story = state.load(story_id).unwrap();
    story.set_body(body);
    Ok(Json(story.clone()))
}

pub async fn add_paragraph(
    Json(content): Json<String>,
    Query(story_id): Query<Ulid>,
    State(state): State<Arc<Mutex<PersistInMemory>>>,
) -> Result<Json<Story>, Error> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind);

    let mut state = state.lock().unwrap();
    let story = state.load(story_id).unwrap();
    let body = story.get_body_mut();
    body.add_paragraph(paragraph);

    Ok(Json(story.clone()))
}

pub async fn report_story(
    Query(story_id): Query<Ulid>,
    State(state): State<Arc<Mutex<PersistInMemory>>>,
) -> Result<Json<Report>, Error> {
    let mut state = state.lock().unwrap();
    let story = state.load(story_id).unwrap();

    let report = Report::new(story.clone());
    Ok(Json(report))
}

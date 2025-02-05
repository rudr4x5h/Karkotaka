use anyhow::Error;
use axum::extract::Json;
use axum::response::IntoResponse;

use crate::core::primary::body::Body;
use crate::core::primary::headline::Headline;
use crate::core::primary::headshot::Headshot;
use crate::core::primary::story::Story;
use crate::core::primary::synopsis::Synopsis;
use crate::core::secondary::image::Image;
use crate::core::secondary::misc::Kind;
use crate::core::secondary::paragraph::Paragraph;
use crate::core::secondary::report::Report;

pub async fn root() -> impl IntoResponse {
    "Namaskaram"
}

pub async fn create_story(Json(content): Json<String>) -> Result<Json<Story>, Error> {
    let kind = Kind::OG;
    let headline = Headline::new(content, kind);
    let story = Story::new(headline);
    Ok(Json(story))
}

pub async fn add_headshot(Json(uri): Json<String>) -> Result<Json<Headshot>, Error> {
    let kind = Kind::OG;
    let image = Image::new(uri);
    let headshot = Headshot::new(kind, image);
    Ok(Json(headshot))
}

pub async fn add_synopsis(Json(content): Json<String>) -> Result<Json<Synopsis>, Error> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind.clone());

    let mut synopsis = Synopsis::new(kind.clone());
    synopsis.add_paragraph(paragraph);
    Ok(Json(synopsis))
}

pub async fn add_body(Json(paragraphs): Json<Vec<Paragraph>>) -> Result<Json<Body>, Error> {
    let body = Body::from_paras(paragraphs);
    Ok(Json(body))
}

pub async fn add_paragraph(Json(content): Json<String>) -> Result<Json<Paragraph>, Error> {
    let kind = Kind::OG;
    let paragraph = Paragraph::new(content, kind);
    Ok(Json(paragraph))
}

pub async fn report_story(Json(story): Json<Story>) -> Result<Json<Report>, Error> {
    let report = Report::new(story);
    Ok(Json(report))
}

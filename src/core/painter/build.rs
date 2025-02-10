use cosmic_text::{Color, Metrics};

use crate::core::primary::{headline::Headline, headshot::Headshot, synopsis::Synopsis};

#[derive(Debug, Clone)]
pub struct Canvas {
    headshot: Headshot,
    headline: Headline,
    synopsis: Synopsis,
    style: Style,
    bounds_primary: Bounds,
    bounds_secondary: Bounds,
}

impl Canvas {
    pub fn headshot(&self) -> &Headshot {
        &self.headshot
    }

    pub fn headline(&self) -> &Headline {
        &self.headline
    }

    pub fn synopsis(&self) -> &Synopsis {
        &self.synopsis
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn bounds_primary(&self) -> &Bounds {
        &self.bounds_primary
    }

    pub fn bounds_secondary(&self) -> &Bounds {
        &self.bounds_secondary
    }
}

pub struct CanvasBuilder {
    headshot: Option<Headshot>,
    headline: Option<Headline>,
    synopsis: Option<Synopsis>,
    style: Option<Style>,
    bounds: Option<Bounds>,
}

#[derive(Clone, Debug)]
pub struct Bounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Bounds {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    text_color: Color,
    highlight_color: Color,
    metrics_primary: Metrics,
    metrics_secondary: Metrics,
}

impl Style {
    pub fn text_color(&self) -> Color {
        self.text_color
    }

    pub fn highlight_color(&self) -> Color {
        self.highlight_color
    }

    pub fn metrics_primary(&self) -> Metrics {
        self.metrics_primary
    }

    pub fn metrics_secondary(&self) -> Metrics {
        self.metrics_secondary
    }
}

impl Builder for CanvasBuilder {
    type OutputType = Canvas;

    fn set_headshot(&mut self, headshot: Headshot) {
        self.headshot = Some(headshot);
    }

    fn set_headline(&mut self, headline: Headline) {
        self.headline = Some(headline);
    }

    fn set_synopsis(&mut self, synopsis: Synopsis) {
        self.synopsis = Some(synopsis);
    }

    fn set_style(
        &mut self,
        font_size: f32,
        line_height: f32,
        p2s_ratio: f32,
        text_color: Color,
        highlight_color: Color,
    ) {
        let primary_metrics = Metrics::new(font_size, line_height);
        let secondary_metrics = Metrics::new(font_size / p2s_ratio, line_height / p2s_ratio);

        let style = Style {
            text_color,
            highlight_color,
            metrics_primary: primary_metrics,
            metrics_secondary: secondary_metrics,
        };

        self.style = Some(style);
    }

    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let bounds = Bounds {
            x,
            y,
            width,
            height,
        };
        self.bounds = Some(bounds);
    }

    fn build(self) -> Self::OutputType {
        Canvas {
            headshot: self.headshot.unwrap(),
            headline: self.headline.unwrap(),
            synopsis: self.synopsis.unwrap(),
            style: self.style.unwrap(),
            bounds_primary: self.bounds.clone().unwrap(),
            bounds_secondary: self.bounds.clone().unwrap(),
        }
    }
}

pub trait Builder {
    type OutputType;
    fn set_headshot(&mut self, headshot: Headshot);
    fn set_headline(&mut self, headline: Headline);
    fn set_synopsis(&mut self, synopsis: Synopsis);
    fn set_style(
        &mut self,
        font_size: f32,
        line_height: f32,
        p2s_ratio: f32,
        text_color: Color,
        highlight_color: Color,
    );
    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn build(self) -> Self::OutputType;
}

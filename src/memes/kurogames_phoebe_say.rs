use rand::RngExt;
use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_core::meme::MemeOption;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::register_meme;

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Number {
    pub number: Option<i32>,
}

impl MemeOptions for Number {
    fn to_options(&self) -> Vec<MemeOption> {
        vec![]
    }
}

const DEFAULT_TEXT: &str = "菲比啾比";

fn kurogames_phoebe_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };

    let num = match options.number {
        Some(n) => n,
        None => rand::rng().random_range(1..=13),
    };

    let frame = load_image(&format!("kurogames_phoebe_say/{}.png", num))?;

    let rects = [
        IRect::from_ltrb(15, 29, 954, 387),
        IRect::from_ltrb(0, 0, 515, 210),
        IRect::from_ltrb(0, 0, 667, 206),
        IRect::from_ltrb(0, 0, 512, 114),
        IRect::from_ltrb(0, 0, 727, 249),
        IRect::from_ltrb(0, 0, 800, 171),
        IRect::from_ltrb(0, 0, 800, 250),
        IRect::from_ltrb(0, 0, 1024, 222),
        IRect::from_ltrb(0, 0, 800, 162),
        IRect::from_ltrb(0, 0, 594, 254),
        IRect::from_ltrb(0, 0, 1024, 217),
        IRect::from_ltrb(0, 0, 361, 147),
        IRect::from_ltrb(0, 0, 626, 149),
    ];

    let index = (num - 1) as usize;
    let rect = rects[index];

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rect,
        text,
        10.0,
        200.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_phoebe_say",
    kurogames_phoebe_say,
    min_images = 0,
    max_images = 0,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["菲比说"],
    date_created = local_date(2025, 5, 10),
    date_modified = local_date(2026, 2, 16),
);
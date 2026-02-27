use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "你是高手？";

fn xueli_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };

    let frame = load_image("xueli_say/0.jpg")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(240, 30, 300, 240),
        text,
        10.0,
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "xueli_say",
    xueli_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["雪莉说", "雪梨说", "橘雪莉说"],
    date_created = local_date(2025, 10, 5),
    date_modified = local_date(2025, 10, 5),
);
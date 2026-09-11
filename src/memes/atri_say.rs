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

use crate::{options::NoOptions, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "好久不见，你长大了呢。";
fn atri_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("atri_say/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(25, 166, 114, 284),
        text,
        20.0,
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
    "atri_say",
    atri_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["亚托莉说"],
    tags = MemeTags::atri(),
    date_created = local_date(2026, 1, 9),
    date_modified = local_date(2026, 1, 9),
);

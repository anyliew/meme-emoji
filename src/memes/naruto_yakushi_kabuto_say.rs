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

const DEFAULT_TEXT: &str = "仙法白激之术";

fn naruto_yakushi_kabuto_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };

    let frame = load_image("naruto_yakushi_kabuto_say/0.png")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1040, 1, 1370, 483),
        text,
        30.0,
        300.0,
        text_params!(
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "naruto_yakushi_kabuto_say",
    naruto_yakushi_kabuto_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["药师兜说", "兜说"],
    date_created = local_date(2026, 1, 9),
    date_modified = local_date(2026, 1, 13),
);
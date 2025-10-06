use skia_safe::{IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "不可以色色";
fn kurogames_verina_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };

    let frame = load_image(format!("kurogames_verina_holdsign/0.png"))?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(475, 525, 790, 775),
        text,
        30.0,
        120.0,
        text_params!(
            text_align = TextAlign::Center,
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(color_from_hex_code("#000000"))
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_verina_holdsign",
    kurogames_verina_holdsign,
    tags = MemeTags::wuthering_waves(),
    min_texts = 1,
    max_texts = 1,
    keywords = &["小维举牌", "维里奈举牌"],
    default_texts = &[DEFAULT_TEXT],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

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

const DEFAULT_TEXT: &str = "嘟嘟嘟说什么呢";
fn kurogames_phoebe_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("kurogames_phoebe_say/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1, 1, 177, 86),
        text,
        10.0,
        120.0,
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_phoebe_say",
    kurogames_phoebe_say,
    min_texts = 0,
    max_texts = 1,
    tags = MemeTags::phoebe(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["菲比说"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

use rand::RngExt;
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

const DEFAULT_TEXT: &str = "希望你开心哦";
fn kurogames_verina_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let num = rand::rng().random_range(0..=1);
    let files = ["0.png", "1.png"];
    let rects = [
        IRect::from_ltrb(520, 30, 735, 375),
        IRect::from_ltrb(30, 70, 230, 410),
    ];
    let frame = load_image(format!("kurogames_verina_say/{}", files[num]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rects[num],
        text,
        10.0,
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_verina_say",
    kurogames_verina_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["小维说", "维里奈说"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2025, 10, 5),
    date_modified = local_date(2025, 10, 5),
);

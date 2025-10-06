use skia_safe::{Color, IRect, textlayout::TextAlign};

use crate::{options::NoOptions, register_meme, tags::MemeTags};
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

const DEFAULT_TEXT: &str = "曾经我是戴上假面的演员，只想要掩饰真相…";
fn mihoyo_funina_card(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("mihoyo_funina_card/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(361, 274, 1392, 940),
        text,
        30.0,
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
    "mihoyo_funina_card",
    mihoyo_funina_card,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::furina(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["芙芙卡片", "芙宁娜卡片", "芙芙酱卡片"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

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

const DEFAULT_TEXT: &str = "用新鲜的肉烹饪出的沙丁布卡~\n我是比较老派的七丘口味\n这道菜可以添加适量的辣椒\n但不要学拉古那人加奇怪的酸味酱啊！";

fn kurogames_abby_write(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("kurogames_abby_write/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(473, 1164, 1671, 1851),
        text,
        30.0,  // min_fontsize
        120.0, // max_fontsize
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_abby_write",
    kurogames_abby_write,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["阿布写信"],
    date_created = local_date(2025, 7, 4),
    date_modified = local_date(2025, 7, 4),
);
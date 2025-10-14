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

const DEFAULT_TEXT: &str = "我才不要在这种时候放弃,即使当不成中忍,我也会通过其他的途径成为火影的,这就是我的忍道";

fn naruro_uzumaki_naruto_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("naruro_uzumaki_naruto_holdsign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(281, 591, 858, 1001),
        text,
        30.0,  // min_fontsize
        120.0, // max_fontsize
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Left,  // 左对齐
            paint = new_paint(Color::from_rgb(72, 44, 41)), // 颜色 #482c29
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "naruro_uzumaki_naruto_holdsign",
    naruro_uzumaki_naruto_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["鸣人举牌", "漩涡鸣人举牌"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
use std::time::{SystemTime, UNIX_EPOCH};
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

const DEFAULT_TEXT: &str = "何意味？";
const IMAGES: [&str; 2] = ["aima_say/0.jpg", "aima_say/1.jpg"];

fn aima_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 基于时间戳的随机选择（不需要 rand 依赖）
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let index = (timestamp % 2) as usize;  // 0 或 1
    let img_path = IMAGES[index];
    
    let frame = load_image(img_path)?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 根据不同的图片设置不同的文本框坐标
    if index == 0 {  // 对应 "aima_say/0.jpg"
        // 第一张图片的文字区域坐标 (40, 35, 140, 210)
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(40, 35, 140, 210),
            text,
            5.0,
            120.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
    } else {  // 对应 "aima_say/1.jpg"
        // 第二张图片的文字区域坐标 (360, 10, 490, 200)
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(360, 10, 490, 200),
            text,
            5.0,
            120.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
    }

    encode_png(surface.image_snapshot())
}

register_meme!(
    "aima_say",
    aima_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["艾玛说", "樱羽艾玛说"],
    date_created = local_date(2025, 10, 5),
    date_modified = local_date(2025, 10, 5),
);
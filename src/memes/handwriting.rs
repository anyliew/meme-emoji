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

const DEFAULT_TEXT: &str = "你好，世界！";

fn handwriting(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { 
        // 截取文本（最大500字符）
        if texts[0].len() > 500 {
            &texts[0][..500]
        } else {
            &texts[0]
        }
    } else { 
        DEFAULT_TEXT 
    };

    // 加载底图模板
    let frame = load_image("handwriting/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 直接在底图上绘制旋转文本
    // 由于缺少旋转功能，我们调整文本框位置和大小来近似效果
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(580, 500, 1800, 1500), // 调整文本框区域
        text,
        50.0,  // 字体大小
        120.0, // 最大字体大小
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)), // 黑色
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "handwriting",
    handwriting,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["手写"],
    date_created = local_date(2025, 6, 11),
    date_modified = local_date(2025, 6, 11),
);
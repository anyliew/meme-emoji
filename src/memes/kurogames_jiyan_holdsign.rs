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

const DEFAULT_TEXT: &str = "气势确有提升，想来你对练兵之道也颇有研究？";

fn kurogames_jiyan_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 加载图片 (注意原Python代码使用的是0.png)
    let frame = load_image("kurogames_jiyan_holdsign/0.png")?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 绘制文字
    // 坐标: (166, 441, 500, 738)
    // 字体大小范围: 30-120
    // 对齐方式: left
    // 字体: FZKaTong-M19S (方正中卡通)
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(166, 441, 500, 738),
        text,
        30.0,   // min_fontsize
        120.0,  // max_fontsize
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_jiyan_holdsign",
    kurogames_jiyan_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["忌炎举牌"],
    date_created = local_date(2025, 12, 5),
    date_modified = local_date(2025, 12, 5),
);
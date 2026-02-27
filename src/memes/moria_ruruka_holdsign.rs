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

const DEFAULT_TEXT: &str = "变身";

fn moria_ruruka_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 加载图片
    let frame = load_image("moria_ruruka_holdsign/0.png")?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 绘制文字
    // 坐标: (210, 714, 656, 985)
    // 字体大小范围: 10-120
    // 对齐方式: center
    // 字体: FZSJ-QINGCRJ
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(210, 714, 656, 985),
        text,
        10.0,   // min_fontsize
        120.0,  // max_fontsize
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "moria_ruruka_holdsign",
    moria_ruruka_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["露露卡举牌", "森亚露露卡举牌"],
    date_created = local_date(2026, 2, 23),
    date_modified = local_date(2026, 2, 23),
);
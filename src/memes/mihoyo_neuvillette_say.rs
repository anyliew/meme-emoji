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

const DEFAULT_TEXT: &str = "潮水啊，我已归来";

fn mihoyo_neuvillette_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 加载图片
    let frame = load_image("mihoyo_neuvillette_say/0.png")?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 绘制文字
    // 坐标: (104, 53, 307, 460)
    // 字体大小范围: 20-120
    // 对齐方式: center
    // 字体: 无指定，使用默认字体
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(104, 53, 307, 460),
        text,
        20.0,   // min_fontsize
        120.0,  // max_fontsize
        text_params!(
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_neuvillette_say",
    mihoyo_neuvillette_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["那维莱特说", "水龙王说"],
    date_created = local_date(2026, 1, 12),
    date_modified = local_date(2026, 1, 12),
);
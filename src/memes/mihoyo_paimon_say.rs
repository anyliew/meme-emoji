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

const DEFAULT_TEXT: &str = "前面的区域\n以后再来探索吧？";

fn mihoyo_paimon_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 加载图片
    let frame = load_image("mihoyo_paimon_say/0.png")?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 绘制文字
    // 坐标: (145, 148, 619, 1117)
    // 字体大小范围: 20-120
    // 对齐方式: center
    // 文字颜色: (94, 94, 94) 灰色
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(145, 148, 619, 1117),
        text,
        20.0,   // min_fontsize
        120.0,  // max_fontsize
        text_params!(
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(94, 94, 94)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_paimon_say",
    mihoyo_paimon_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["派蒙说"],
    date_created = local_date(2026, 1, 12),
    date_modified = local_date(2026, 1, 12),
);
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

const DEFAULT_TEXT: &str = "嘟嘟嘟说什么呢";

fn kurogames_chisa_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 加载图片
    let frame = load_image("kurogames_chisa_say/0.jpg")?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 绘制文字
    // 坐标: (20, 20, 770, 365)
    // 字体大小范围: 50-120
    // 对齐方式: center
    // 字体: FZShaoEr-M11S
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 20, 770, 365),
        text,
        50.0,   // min_fontsize
        120.0,  // max_fontsize
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_chisa_say",
    kurogames_chisa_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["千咲说"],
    date_created = local_date(2025, 5, 10),
    date_modified = local_date(2025, 5, 10),
);
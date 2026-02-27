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

const DEFAULT_TEXT: &str = "岁主在上\n菲比湫比";

fn kurogames_phoebe_trumpet(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 加载图片
    let frame = load_image("kurogames_phoebe_trumpet/0.png")?;
    
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 绘制文字
    // 坐标: (0, 0, 1024, 290)
    // 字体大小范围: 30-120
    // 对齐方式: center
    // 字体: FZShaoEr-M11S
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 1024, 290),
        text,
        30.0,   // min_fontsize
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
    "kurogames_phoebe_trumpet",
    kurogames_phoebe_trumpet,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["菲比喇叭"],
    date_created = local_date(2026, 1, 23),
    date_modified = local_date(2026, 1, 23),
);
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

const DEFAULT_TEXT: &str = "月相轮转之间，我以我为锚点";

fn kurogames_iuno_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 随机选择两张图片中的一张 (0.png 或 1.png)
    let image_choice = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::SystemTime;
        
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        if hasher.finish() % 2 == 0 { "0.png" } else { "1.png" }
    };
    
    let frame = load_image(&format!("kurogames_iuno_holdsign/{}", image_choice))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 根据图片选择不同的文字区域坐标
    let text_area = if image_choice == "0.png" {
        IRect::from_ltrb(249, 577, 768, 952)  // 第一张图片的文字区域坐标
    } else {
        IRect::from_ltrb(273, 521, 726, 906)  // 第二张图片的文字区域坐标
    };
    
    canvas.draw_text_area_auto_font_size(
        text_area,
        text,
        15.0,  // min_fontsize
        120.0, // max_fontsize
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_iuno_holdsign",
    kurogames_iuno_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["尤诺举牌"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
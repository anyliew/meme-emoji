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

const DEFAULT_TEXT: &str = "佩洛肚皮空空，灵感快来快来";

fn kurogames_roccia_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    
    // 随机选择一张图片 (0, 1, 2)
    let img_index = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::SystemTime;
        
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        hasher.finish() as usize % 3  // 移除不必要的括号
    };
    
    // 为每张图片设置不同的文字区域坐标
    let text_areas = [
        IRect::from_ltrb(313, 548, 623, 902),   // 图片0的坐标
        IRect::from_ltrb(272, 477, 723, 812),   // 图片1的坐标
        IRect::from_ltrb(339, 611, 717, 891)    // 图片2的坐标
    ];
    
    let frame = load_image(&format!("kurogames_roccia_holdsign/{}.jpg", img_index))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    canvas.draw_text_area_auto_font_size(
        text_areas[img_index],
        text,
        30.0,  // min_fontsize
        120.0, // max_fontsize
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_roccia_holdsign",
    kurogames_roccia_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["洛可可举牌"],
    date_created = local_date(2025, 8, 10),
    date_modified = local_date(2025, 8, 10),
);
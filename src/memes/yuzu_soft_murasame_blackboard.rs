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

use crate::{options::NoOptions, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "不要再涩涩了";
fn yuzu_soft_murasame_blackboard(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("yuzu_soft_murasame_blackboard/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(351, 128, 1119, 326),
        text,
        25.0,
        150.0,
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(255, 255, 255))
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "yuzu_soft_murasame_blackboard",
    yuzu_soft_murasame_blackboard,
    tags = MemeTags::murasame(),
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["丛雨黑板"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

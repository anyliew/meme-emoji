use skia_safe::{Color, IRect, textlayout::TextAlign};

use crate::{options::NoOptions, register_meme, tags::MemeTags};
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

const DEFAULT_TEXT: &str = "我希望…终有一天，我能用自己全部的时间去画我真正想画的，在这个世界上留下真正的我的痕迹。";
fn kurogames_zhezhi_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("kurogames_zhezhi_holdsign/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(657, 866, 1907, 1773),
        text,
        30.0,
        120.0,
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_zhezhi_holdsign",
    kurogames_zhezhi_holdsign,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::zhezhi(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["折枝举牌"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

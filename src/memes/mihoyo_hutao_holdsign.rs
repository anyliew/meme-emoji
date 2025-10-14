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

const DEFAULT_TEXT: &str = "嗯哼，太阳出来我晒太阳，月亮出来我晒月亮嘞";

fn mihoyo_hutao_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("mihoyo_hutao_holdsign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(196, 736, 739, 943),
        text,
        30.0,  // min_fontsize
        120.0, // max_fontsize
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_hutao_holdsign",
    mihoyo_hutao_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["胡桃举牌"],
    date_created = local_date(2025, 7, 1),
    date_modified = local_date(2025, 7, 1),
);
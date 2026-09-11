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

const DEFAULT_TEXT: &str = "诶？！！！等……等等等，是一辈子喔？";
fn mygo_chihaya_anon_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("mygo_chihaya_anon_say/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(205, 166, 912, 557),
        text,
        10.0,
        1150.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "mygo_chihaya_anon_say",
    mygo_chihaya_anon_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["千早爱音说"],
    date_created = local_date(2026, 8, 13),
    date_modified = local_date(2026, 9, 4),
);

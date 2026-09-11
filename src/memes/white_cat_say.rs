use rand::RngExt;
use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::number_option, register_meme};

number_option!(Number, 1, 2);

fn white_cat_say(_: Vec<InputImage>, texts: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=2));
    let index = (num as usize) - 1;
    let files = ["0.png", "1.png"];
    let rects = [
        IRect::from_ltrb(55, 20, 215, 91),
        IRect::from_ltrb(100, 16, 488, 188),
    ];
    let frame = load_image(format!("white_cat_say/{}", files[index]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rects[index],
        text,
        10.0,
        200.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "white_cat_say",
    white_cat_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["宝贝~"],
    keywords = &["白猫说"],
    date_created = local_date(2026, 5, 26),
    date_modified = local_date(2026, 5, 26),
);

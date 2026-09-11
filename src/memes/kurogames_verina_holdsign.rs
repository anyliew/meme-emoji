use rand::RngExt;
use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{options::number_option, register_meme, tags::MemeTags};

number_option!(Number, 1, 5);

const DEFAULT_TEXT: &str = "希望你开心哦";

fn kurogames_verina_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=5));
    let index = num as usize - 1;
    let rects = [
        IRect::from_ltrb(475, 525, 790, 775),
        IRect::from_ltrb(345, 730, 645, 960),
        IRect::from_ltrb(50, 650, 710, 760),
        IRect::from_ltrb(380, 660, 670, 890),
        IRect::from_ltrb(330, 680, 670, 950),
    ];

    let frame = load_image(format!("kurogames_verina_holdsign/{index}.png"))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    if index >= 2 {
        canvas.draw_text_area_auto_font_size(
            rects[index],
            text,
            10.0,
            120.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::WHITE, 2.0),
            ),
        )?;
    } else {
        canvas.draw_text_area_auto_font_size(
            rects[index],
            text,
            10.0,
            120.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
    }

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_verina_holdsign",
    kurogames_verina_holdsign,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["小维举牌", "维里奈举牌"],
    date_created = local_date(2025, 10, 5),
    date_modified = local_date(2026, 7, 21),
);

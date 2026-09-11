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

number_option!(Number, 1, 3);

fn mygo_togawa_sakiko_say(_: Vec<InputImage>, texts: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=3));
    let index = (num as usize) - 1;
    let files = ["0.png", "1.png", "2.png"];
    let rects = [
        IRect::from_ltrb(1483, 172, 2318, 438),
        IRect::from_ltrb(1145, 101, 1893, 420),
        IRect::from_ltrb(1403, 99, 2374, 1009),
    ];
    let frame = load_image(format!("mygo_togawa_sakiko_say/{}", files[index]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rects[index],
        text,
        10.0,
        180.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "mygo_togawa_sakiko_say",
    mygo_togawa_sakiko_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我从没觉得玩乐队开心过……"],
    keywords = &["丰川祥子说", "祥子说"],
    date_created = local_date(2026, 8, 13),
    date_modified = local_date(2026, 8, 13),
);

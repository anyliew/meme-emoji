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

number_option!(Number, 1, 4);

fn mygo_wakaba_mutsumi_say(_: Vec<InputImage>, texts: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=4));
    let index = (num as usize) - 1;
    let files = ["0.png", "1.png", "2.png", "3.png"];
    let rects = [
        IRect::from_ltrb(31, 89, 166, 175),
        IRect::from_ltrb(40, 48, 192, 139),
        IRect::from_ltrb(24, 33, 180, 140),
        IRect::from_ltrb(27, 47, 172, 141),
    ];
    let frame = load_image(format!("mygo_wakaba_mutsumi_say/{}", files[index]))?;
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
    "mygo_wakaba_mutsumi_say",
    mygo_wakaba_mutsumi_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我从没觉得玩乐队开心过……"],
    keywords = &["若叶睦说", "睦子米说"],
    date_created = local_date(2026, 4, 11),
    date_modified = local_date(2026, 6, 10),
);

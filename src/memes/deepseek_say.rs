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

fn deepseek_say(_: Vec<InputImage>, texts: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=2));
    let index = (num as usize) - 1;
    let files = ["0.png", "1.png"];
    let rects = [
        IRect::from_ltrb(59, 57, 569, 336),
        IRect::from_ltrb(139, 140, 653, 427),
    ];
    let colors = [Color::from_rgb(4, 21, 70), Color::from_rgb(16, 49, 94)];
    let frame = load_image(format!("deepseek_say/{}", files[index]))?;
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
            paint = new_paint(colors[index]),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "deepseek_say",
    deepseek_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["正在思考中……"],
    keywords = &["Deepseek说", "deepseek说", "鲸鱼娘说"],
    date_created = local_date(2026, 8, 24),
    date_modified = local_date(2026, 8, 24),
);

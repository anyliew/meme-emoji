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

use crate::{options::number_option, register_meme, tags::MemeTags};

number_option!(Number, 1, 2);

const DEFAULT_TEXT: &str = "我看到弹幕上的好好好……";

fn kurogames_songlun_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=2));
    let index = num as usize - 1;
    let files = ["0.png", "1.png"];
    let rects = [
        IRect::from_ltrb(283, 318, 760, 615),
        IRect::from_ltrb(280, 185, 942, 545),
    ];

    let frame = load_image(format!("kurogames_songlun_say/{}", files[index]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rects[index],
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
    "kurogames_songlun_say",
    kurogames_songlun_say,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["难道说", "松伦说"],
    date_created = local_date(2025, 6, 10),
    date_modified = local_date(2025, 6, 10),
);

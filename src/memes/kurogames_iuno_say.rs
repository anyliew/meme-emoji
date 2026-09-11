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

const DEFAULT_TEXT: &str = "月亮游离世间";

fn kurogames_iuno_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=2));
    let index = num as usize - 1;
    let files = ["0.jpg", "1.jpg"];
    let rects = [
        IRect::from_ltrb(1, 1, 199, 42),
        IRect::from_ltrb(0, 0, 1024, 249),
    ];

    let frame = load_image(format!("kurogames_iuno_say/{}", files[index]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rects[index],
        text,
        30.0,
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
    "kurogames_iuno_say",
    kurogames_iuno_say,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["尤诺说"],
    date_created = local_date(2025, 8, 11),
    date_modified = local_date(2025, 8, 11),
);

use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn bully_me(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let img = images[0].image.resize_width(650);
    let display_text = format!("当你想对{text}恶语相向时，请注意，屏幕后面的我可是这样的：");
    let text_height = 50 * (display_text.chars().count() as i32 / 15 + 1) + 10;
    let mut surface = new_surface((img.width(), text_height + img.height()));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 10, img.width() - 20, text_height - 10),
        &display_text,
        20.0,
        40.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    canvas.draw_image(&img, (0, text_height), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "bully_me",
    bully_me,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我"],
    keywords = &["恶语相向"],
    date_created = local_date(2025, 10, 11),
    date_modified = local_date(2025, 10, 11),
);

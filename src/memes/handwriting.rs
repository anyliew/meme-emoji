use skia_safe::{Color, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn handwriting(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("handwriting/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.rotate(-17.0, None);
    let mut text_img = Text2Image::from_text(
        text,
        200.0,
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::BLACK)
        ),
    );
    text_img.layout(2360.0);
    text_img.draw_on_canvas(canvas, (240, 1210));
    canvas.reset_matrix();
    encode_png(surface.image_snapshot())
}

register_meme!(
    "handwriting",
    handwriting,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["你好，世界！"],
    keywords = &["手写"],
    date_created = local_date(2025, 8, 10),
    date_modified = local_date(2025, 8, 10),
);

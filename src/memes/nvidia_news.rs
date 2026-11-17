use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nvidia_news(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else if !images[0].name.is_empty() {
        images[0].name.clone()
    } else {
        "他".to_string()
    };
    let text = format!("刚刚得知：{name}据报道被提名作为英伟达CEO黄仁勋的潜在继任者。");
    let frame = load_image("nvidia_news/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(145, 130, 936, 230),
        &text,
        20.0,
        25.0,
        text_params!(
            text_align = TextAlign::Left,
            paint = new_paint(Color::BLACK)
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0].resize_fit((386, 540), Fit::Cover);
        canvas.draw_image(&img, (155, 244), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "nvidia_news",
    nvidia_news,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["n卡新闻", "nvidia新闻", "N卡新闻", "Nvidia新闻"],
    date_created = local_date(2026, 11, 13),
    date_modified = local_date(2026, 11, 13),
);

use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn contract(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name}の奴隶契约");
    let frame = load_image("contract/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1416, 515, 1905, 597),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0))
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_bound((135, 135), Fit::Cover);
        canvas.draw_image(&img, (1770, 1096), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "contract",
    contract,
    min_images = 1,
    max_images = 1,
    keywords = &["卖身契","⭐️💢契约","奴隶契约",],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

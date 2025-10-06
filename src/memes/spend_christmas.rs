use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn spend_christmas(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name}，我们一起过圣诞");
    let frame = load_image("spend_christmas/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(173, 193, 728, 374),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(57, 49, 46))
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((255, 255));
        canvas.draw_image(&img, (120, 535), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "spend_christmas",
    spend_christmas,
    min_images = 1,
    max_images = 1,
    keywords = &["一起圣诞"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

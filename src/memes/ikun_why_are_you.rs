use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn ikun_why_are_you(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("ikun_why_are_you/0.jpg")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((120, 120)).rotate(-10.0);
        canvas.draw_image(&image, (275, 75), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "ikun_why_are_you",
    ikun_why_are_you,
    min_images = 1,
    max_images = 1,
    keywords = &["你干嘛", "你干吗"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn anyliew_struggling(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("anyliew_struggling/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_bound((300, 300), Fit::Cover).rotate(-25.0);
        canvas.draw_image(&image, (275, 28), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "anyliew_struggling",
    anyliew_struggling,
    min_images = 1,
    max_images = 1,
    keywords = &["挣扎"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

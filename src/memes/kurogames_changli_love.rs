use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_changli_love(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_changli_love/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        let image = images[0].circle().resize_fit((108, 90), Fit::Cover);
        canvas.draw_image(&image, (115, 194), None);
        canvas.draw_image(&frame, (0, 0), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_changli_love",
    kurogames_changli_love,
    min_images = 1,
    max_images = 1,
    keywords = &["长离爱心"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 8, 24),
    date_modified = local_date(2026, 8, 24),
);

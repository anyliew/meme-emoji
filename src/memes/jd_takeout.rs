use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn jd_takeout(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("jd_takeout/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].rotate(-10.0).resize_bound((260, 260), Fit::Cover).rotate(10.0);
        canvas.draw_image(&image, (610, -13), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "jd_takeout",
    jd_takeout,
    min_images = 1,
    max_images = 1,
    keywords = &["京东外卖"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn error3b(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("error3b/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        let image = images[0].resize_exact((1024, 1024));
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&frame, (0, 0), None);

        Ok(surface.image_snapshot().round_corner(200.0))
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "error3b",
    error3b,
    min_images = 1,
    max_images = 1,
    keywords = &["30亿error", "三十亿报错"],
    date_created = local_date(2026, 4, 17),
    date_modified = local_date(2026, 4, 17),
);

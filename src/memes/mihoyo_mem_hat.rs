use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mihoyo_mem_hat(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_mem_hat/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        let image = images[0].circle().resize_exact((245, 245));
        canvas.draw_image(&image, (145, 155), None);
        canvas.draw_image(&frame, (0, 0), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_mem_hat",
    mihoyo_mem_hat,
    min_images = 1,
    max_images = 1,
    keywords = &["迷迷帽", "迷迷帽子"],
    tags = MemeTags::star_rail(),
    date_created = local_date(2026, 4, 17),
    date_modified = local_date(2026, 4, 17),
);

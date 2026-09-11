use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_lantern_festival(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_lantern_festival/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        let image = images[0].circle().resize_exact((375, 375));
        canvas.draw_image(&image, (62, 92), None);
        canvas.draw_image(&frame, (0, 0), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_lantern_festival",
    kurogames_lantern_festival,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["元宵"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 4, 8),
    date_modified = local_date(2026, 4, 8),
);

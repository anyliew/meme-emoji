use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_rover_head(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_rover_head/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].circle().resize_bound((1200, 1200), Fit::Cover).rotate(-10.0);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&image, (248, 250), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_rover_head",
    kurogames_rover_head,
    tags = MemeTags::rover(),
    min_images = 1,
    max_images = 1,
    keywords = &["漂泊者头像框"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

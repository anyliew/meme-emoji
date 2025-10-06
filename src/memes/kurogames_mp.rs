use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_mp(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_mp/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].circle().resize_bound((130, 130), Fit::Cover).rotate(-25.0);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&image, (300, 2), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_mp",
    kurogames_mp,
    tags = MemeTags::wuthering_waves(),
    min_images = 1,
    max_images = 1,
    keywords = &["鸣批", "鸣P", "鸣p", "鸣潮玩家", "鸣潮男"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

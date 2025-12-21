use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme,  tags::MemeTags};

fn kurogames_camellya_red_hug(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_camellya_red_hug/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((428, 428), Fit::Cover);
        canvas.draw_image(&image, (399, 0), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_camellya_red_hug",
    kurogames_camellya_red_hug,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::wuthering_waves(),
    keywords = &["红椿抱"],
    date_created = local_date(2025, 12, 19),
    date_modified = local_date(2025, 12, 19),
);

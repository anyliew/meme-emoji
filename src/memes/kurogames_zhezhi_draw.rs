use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_zhezhi_draw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_zhezhi_draw/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((370, 370), Fit::Cover).rotate(30.0);
        canvas.draw_image(&image, (435, 390), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_zhezhi_draw",
    kurogames_zhezhi_draw,
    min_images = 1,
    max_images = 1,
    keywords = &["折枝画画","折枝绘画"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

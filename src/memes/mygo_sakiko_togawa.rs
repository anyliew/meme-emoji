use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mygo_sakiko_togawa(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("mygo_sakiko_togawa/0.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].resize_fit((150, 150), Fit::Cover).rotate(-5.0);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (105, 70), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mygo_sakiko_togawa",
    mygo_sakiko_togawa,
    tags = MemeTags::sakiko_togawa(),
    min_images = 1,
    max_images = 1,
    keywords = &["丰川祥子", "祥子", "豊川祥子"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

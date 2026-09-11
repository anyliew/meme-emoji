use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn gong_xi_fa_cai(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("gong_xi_fa_cai/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        canvas.draw_image(&frame, (0, 0), None);
        let image = images[0].circle().resize_exact((800, 800));
        canvas.draw_image(&image, (1035, 700), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "gong_xi_fa_cai",
    gong_xi_fa_cai,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["恭喜发财"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);

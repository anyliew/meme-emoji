use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_funina_finger(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_funina_finger/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_bound((500, 500), Fit::Cover);
        canvas.draw_image(&image, (1400, 650), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_funina_finger",
    mihoyo_funina_finger,
    min_images = 1,
    max_images = 1,
    keywords = &["芙芙指","芙宁娜指","芙芙酱指"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn catch(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("catch/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        let image = images[0].circle().resize_exact((999, 999));
        canvas.draw_image(&image, (73, 93), None);
        canvas.draw_image(&frame, (0, 0), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "catch",
    catch,
    min_images = 1,
    max_images = 1,
    keywords = &["抓住"],
    date_created = local_date(2026, 4, 17),
    date_modified = local_date(2026, 4, 17),
);

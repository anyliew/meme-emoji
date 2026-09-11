use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ending(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("ending/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(
        &images[1].image.circle().resize_exact((284, 284)),
        (463, 700),
        None,
    );
    canvas.draw_image(
        &images[0].image.circle().resize_exact((530, 530)),
        (1516, 880),
        None,
    );
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "ending",
    ending,
    min_images = 2,
    max_images = 2,
    keywords = &["下场"],
    date_created = local_date(2026, 5, 17),
    date_modified = local_date(2026, 5, 17),
);

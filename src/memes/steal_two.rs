use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn steal_two(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("steal_two/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(
        &images[0].image.circle().resize_exact((131, 131)),
        (253, 78),
        None,
    );
    canvas.draw_image(
        &images[1].image.circle().resize_exact((140, 140)),
        (11, 304),
        None,
    );
    canvas.draw_image(
        &images[2].image.circle().resize_exact((125, 125)),
        (532, 387),
        None,
    );
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "steal_two",
    steal_two,
    min_images = 3,
    max_images = 3,
    keywords = &["双偷"],
    date_created = local_date(2026, 4, 11),
    date_modified = local_date(2026, 4, 17),
);

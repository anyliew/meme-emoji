use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn torture_yourself(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("torture_yourself/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].resize_fit((175, 175), Fit::Cover);
        let image2 = images[1].resize_fit((380, 410), Fit::Cover);
        canvas.draw_image(&image1, (722, 752), None);
        canvas.draw_image(&image2, (63, 278), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "torture_yourself",
    torture_yourself,
    min_images = 2,
    max_images = 2,
    keywords = &["折磨自己"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

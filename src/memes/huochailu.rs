use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn huochailu(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (155, 155, 63, 28),
        (155, 155, 63, 28),
        (155, 155, 83, 38),
        (155, 155, 94, 40),
        (155, 155, 97, 45),
        (155, 155, 97, 45),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("huochailu/{i}.png"))?;
        let (w, h, x, y) = locs[i];
        let image = images[0].resize_bound((w, h), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 5, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "huochailu",
    huochailu,
    min_images = 1,
    max_images = 1,
    keywords = &["火柴撸"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

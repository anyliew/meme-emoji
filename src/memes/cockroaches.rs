use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn cockroaches(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("cockroaches/{i}.png"))?;
        let user_head = images[0].resize_fit((115, 90), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&user_head, (94, 95), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.09,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "cockroaches",
    cockroaches,
    min_images = 1,
    max_images = 1,
    keywords = &["蟑螂", "小强"],
    date_created = local_date(2025, 7, 1),
    date_modified = local_date(2026, 6, 10),
);

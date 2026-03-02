use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn christmas_gift(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (0, 0),
        (0, 244),
        (0, 197),
        (0, 159), // 0-3
        (0, 129),
        (0, 105),
        (0, 87),
        (0, 74),
        (0, 67), // 4-8
        (0, 61),
        (0, 57),
        (0, 53),
        (0, 51),
        (0, 49), // 9-13
        (0, 48),
        (0, 46),
        (0, 48),
        (0, 51),
        (0, 56), // 14-18
        (0, 62),
        (0, 66),
        (0, 67),
        (0, 67),
        (0, 67), // 19-23
        (0, 65),
        (0, 65),
        (0, 61),
        (0, 60),
        (0, 60), // 24-28
        (0, 60),
        (0, 67),
        (0, 92),
        (0, 141),
        (0, 201), // 29-33
        (0, 245),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0), // 34-38
        (0, 0),
        (0, 0), // 39-40
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("christmas_gift/{i}.png"))?;
        let user_head = images[0].resize_fit((300, 215), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if positions[i].1 != 0 {
            canvas.draw_image(&user_head, positions[i], None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 41,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "christmas_gift",
    christmas_gift,
    min_images = 1,
    max_images = 1,
    keywords = &["圣诞礼物"],
    date_created = local_date(2025, 12, 23),
    date_modified = local_date(2025, 12, 23),
);

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ikun_ball(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(40, 44); 2];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("ikun_ball/{i}.png"))?;
        let user_head = images[0].resize_fit((145, 107), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 2,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "ikun_ball",
    ikun_ball,
    min_images = 1,
    max_images = 1,
    keywords = &["坤球"],
    date_created = local_date(2026, 5, 26),
    date_modified = local_date(2026, 5, 26),
);

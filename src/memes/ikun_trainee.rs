use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ikun_trainee(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (58, 52), (56, 55), (59, 52), (60, 51), (59, 45),
        (62, 42), (63, 48), (60, 53), (63, 48), (62, 42),
        (59, 46), (60, 51),
    ];
    let sizes = [
        (64, 42), (65, 42), (64, 42), (61, 43), (63, 51),
        (58, 50), (57, 49), (61, 43), (56, 49), (59, 50),
        (63, 50), (61, 43),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("ikun_trainee/{i}.png"))?;
        let user_head = images[0].resize_fit(sizes[i], Fit::Cover);
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
            frame_num: 12,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "ikun_trainee",
    ikun_trainee,
    min_images = 1,
    max_images = 1,
    keywords = &["练习生"],
    date_created = local_date(2026, 4, 8),
    date_modified = local_date(2026, 4, 8),
);

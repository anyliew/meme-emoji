use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn christmas_eve(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("christmas_eve/{i}.png"))?;
        let user_head = images[0].resize_exact((118, 89));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, (70, 120), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 3,
            duration: 0.5,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "christmas_eve",
    christmas_eve,
    min_images = 1,
    max_images = 1,
    keywords = &["平安夜","平安夜快乐"],
    date_created = local_date(2025, 8, 18),
    date_modified = local_date(2025, 12, 9),
);

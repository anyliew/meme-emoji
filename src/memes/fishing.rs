use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn fishing(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("fishing/{i}.png"))?;
        let user_head = images[0].resize_exact((100, 70));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, (104, 89), None);
        canvas.draw_image(&frame, (0,0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 8,
            duration: 0.11,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "fishing",
    fishing,
    min_images = 1,
    max_images = 1,
    keywords = &["钓鱼"],
    date_created = local_date(2025, 8, 19),
    date_modified = local_date(2025, 9, 1),
);

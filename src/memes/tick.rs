use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn tick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(64, 64); 30];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("tick/{i}.png"))?;
        let user_head = images[0].resize_exact((100, 100)).circle();
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
            frame_num: 30,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "tick",
    tick,
    min_images = 1,
    max_images = 1,
    keywords = &["虫"],
    date_created = local_date(2026, 4, 11),
    date_modified = local_date(2026, 4, 11),
);

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ace_taffy_sunglasses(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(19, 19); 24];
    let sizes = [(187, 187); 24];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("ace_taffy_sunglasses/{i}.png"))?;
        let head = images[0].square().resize_fit(sizes[i], Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 24,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "ace_taffy_sunglasses",
    ace_taffy_sunglasses,
    min_images = 1,
    max_images = 1,
    keywords = &["太阳镜"],
    date_created = local_date(2026, 8, 2),
    date_modified = local_date(2026, 8, 2),
);

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_denia_hug(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (60, 146), (60, 146), (61, 145), (61, 145), (61, 145),
        (61, 145), (61, 145), (61, 145), (61, 145), (61, 145),
        (61, 145), (61, 145), (61, 145), (61, 145), (60, 146),
        (60, 146), (60, 146), (60, 145), (60, 145), (60, 146),
        (60, 146), (60, 146), (60, 146), (60, 146), (60, 145),
        (60, 145), (60, 145), (61, 145), (61, 145), (60, 146),
        (60, 146), (60, 146), (59, 146), (59, 146), (60, 145),
        (60, 145), (60, 145), (61, 145), (61, 145), (60, 145),
        (60, 145), (60, 145), (60, 146), (60, 146), (61, 144),
        (61, 144), (61, 144), (60, 145), (60, 145), (60, 145),
    ];
    let sizes = [
        (81, 54), (81, 54), (80, 55), (80, 55), (80, 55),
        (80, 55), (80, 55), (80, 55), (80, 55), (80, 55),
        (80, 55), (80, 55), (80, 55), (80, 55), (81, 54),
        (81, 54), (81, 54), (80, 55), (80, 55), (81, 54),
        (81, 54), (81, 54), (80, 54), (80, 54), (82, 55),
        (82, 55), (82, 55), (80, 55), (80, 55), (81, 54),
        (81, 54), (81, 54), (82, 54), (82, 54), (81, 55),
        (81, 55), (81, 55), (79, 55), (79, 55), (82, 55),
        (82, 55), (82, 55), (81, 54), (81, 54), (80, 56),
        (80, 56), (80, 56), (81, 55), (81, 55), (82, 55),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_denia_hug/{i}.png"))?;
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
            frame_num: 50,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_denia_hug",
    kurogames_denia_hug,
    min_images = 1,
    max_images = 1,
    keywords = &["达妮娅抱"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 9, 3),
    date_modified = local_date(2026, 9, 3),
);

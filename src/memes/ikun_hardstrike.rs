use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ikun_hardstrike(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (111, 17), (112, 16), (113, 16), (111, 15),
    ];
    let sizes = [
        (59, 49), (60, 52), (59, 50), (60, 50),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("ikun_hardstrike/{i}.png"))?;
        let head = images[0].square().resize_exact(sizes[i]);
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
            frame_num: 4,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "ikun_hardstrike",
    ikun_hardstrike,
    min_images = 1,
    max_images = 1,
    keywords = &["铁山靠"],
    date_created = local_date(2026, 7, 23),
    date_modified = local_date(2026, 7, 23),
);

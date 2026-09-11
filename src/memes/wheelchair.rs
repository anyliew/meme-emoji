use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn wheelchair(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [
        (303, 117),
        (303, 117),
        (303, 117),
        (303, 117),
        (304, 117),
        (313, 120),
        (330, 126),
        (352, 139),
        (366, 153),
        (365, 157),
        (353, 154),
        (330, 140),
        (309, 126),
        (304, 120),
        (304, 117),
        (303, 117),
        (302, 117),
        (302, 117),
        (303, 117),
        (303, 117),
    ];
    let user_locs = [
        (167, 250),
        (167, 250),
        (167, 250),
        (167, 250),
        (168, 250),
        (173, 250),
        (181, 250),
        (186, 250),
        (185, 249),
        (180, 249),
        (169, 250),
        (161, 250),
        (157, 250),
        (161, 250),
        (167, 250),
        (167, 250),
        (167, 250),
        (167, 250),
        (167, 250),
        (167, 250),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("wheelchair/{i}.png"))?;
        let self_head = images[0].resize_fit((54, 54), Fit::Cover).rotate(30.0);
        let user_head = images[1]
            .resize_fit((124, 124), Fit::Cover)
            .rotate(15.0 * (i as f32 + 1.0));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 20,
            duration: 0.02,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "wheelchair",
    wheelchair,
    min_images = 2,
    max_images = 2,
    keywords = &["轮椅"],
    date_created = local_date(2026, 5, 26),
    date_modified = local_date(2026, 5, 26),
);

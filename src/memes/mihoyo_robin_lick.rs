use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mihoyo_robin_lick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (244, 57), (244, 58), (244, 57), (244, 56), (244, 57),
        (244, 57), (244, 57), (223, 58), (254, 57), (223, 58),
        (254, 57), (223, 58), (254, 57), (223, 58), (254, 57),
        (191, 62), (235, 62), (251, 56), (258, 60), (262, 57),
        (263, 56), (264, 55), (257, 56), (241, 56), (241, 60),
        (243, 58), (244, 57), (244, 57),
    ];
    let sizes = [
        (184, 147), (184, 148), (184, 147), (184, 148), (184, 147),
        (184, 147), (184, 147), (173, 140), (182, 148), (172, 140),
        (182, 148), (172, 140), (182, 148), (172, 140), (182, 148),
        (169, 143), (180, 138), (182, 149), (184, 145), (184, 147),
        (184, 148), (184, 151), (184, 148), (184, 148), (184, 146),
        (184, 146), (184, 147), (184, 147),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("mihoyo_robin_lick/{i}.png"))?;
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
            frame_num: 28,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "mihoyo_robin_lick",
    mihoyo_robin_lick,
    min_images = 1,
    max_images = 1,
    keywords = &["知更鸟舔"],
    tags = MemeTags::star_rail(),
    date_created = local_date(2026, 9, 9),
    date_modified = local_date(2026, 9, 9),
);

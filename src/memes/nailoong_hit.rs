use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_hit(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (107, 101), (106, 101), (105, 100), (129, 116), (133, 117),
        (133, 115), (128, 113), (125, 110), (125, 110), (124, 110),
        (124, 110), (123, 110), (123, 110), (122, 109), (121, 109),
        (114, 104), (112, 103), (111, 103), (112, 104), (111, 101),
        (111, 101), (111, 102), (111, 104), (112, 103), (112, 103),
        (113, 104), (113, 104), (113, 104), (112, 104), (112, 104),
        (112, 104),
    ];
    let sizes = [(60, 60); 31];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_hit/{i}.png"))?;
        let user_head = images[0].resize_exact(sizes[i]);
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
            frame_num: 31,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_hit",
    nailoong_hit,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙打"],
    date_created = local_date(2026, 7, 13),
    date_modified = local_date(2026, 7, 13),
);

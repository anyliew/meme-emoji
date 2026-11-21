use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_steady(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let positions = [
        (86, 83), (94, 1), (90, -2), (81, -3), (63, -3),
        (55, -3), (41, -3), (34, 0), (31, 5), (31, 11),
        (32, 15), (42, 30), (58, 50), (66, 64), (70, 76),
        (71, 88), (73, 90), (78, 92), (88, 93), (92, 95),
        (92, 88), (92, 85), (83, 70), (77, 56), (72, 46),
        (66, 33), (63, 29), (59, 20), (49, 23), (42, 20),
    ];
    let sizes = [
        (63, 63), (52, 52), (52, 52), (52, 47), (57, 49),
        (57, 49), (57, 52), (60, 60), (60, 60), (63, 63),
        (63, 63), (63, 63), (63, 63), (63, 63), (55, 55),
        (55, 55), (55, 55), (55, 55), (55, 55), (60, 60),
        (63, 63), (63, 63), (70, 70), (76, 76), (76, 76),
        (79, 79), (86, 86), (86, 86), (93, 93), (93, 93),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_steady/{i}.png"))?;
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
            frame_num: 30,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_steady",
    nailoong_steady,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙扶车", "扶车"],
    date_created = local_date(2026, 11, 21),
    date_modified = local_date(2026, 11, 21),
);

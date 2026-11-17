use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_scratchead(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let positions = [
        (266, 35), (266, 35), (266, 35), (266, 35), (265, 35),
        (265, 35), (263, 35), (261, 35), (258, 35), (251, 30),
        (248, 30), (244, 27), (241, 27), (238, 26), (233, 24),
        (230, 23), (226, 21), (221, 20), (222, 20), (223, 19),
        (219, 19), (217, 18), (217, 19), (223, 18), (221, 18),
        (215, 18), (219, 17), (216, 17), (214, 17), (215, 15),
        (216, 15), (213, 15), (213, 15), (217, 13), (222, 13),
        (222, 13), (226, 13), (228, 13),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_scratchead/{i}.png"))?;
        let user_head = images[0].resize_exact((78, 78));
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
            frame_num: 38,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_scratchead",
    nailoong_scratchead,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙挠痒痒", "挠痒痒"],
    date_created = local_date(2026, 11, 17),
    date_modified = local_date(2026, 11, 17),
);

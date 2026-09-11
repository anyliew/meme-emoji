use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_kiss(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (68, 57), (67, 57), (69, 54), (81, 51), (98, 57),
        (99, 57), (99, 57), (95, 55), (62, 46), (60, 45),
        (60, 45), (49, 48), (46, 48), (38, 50), (38, 51),
        (30, 61), (27, 63),
    ];
    let sizes = [
        (69, 69), (69, 69), (69, 69), (69, 69), (69, 69),
        (69, 69), (69, 69), (69, 69), (69, 69), (69, 69),
        (69, 69), (76, 76), (76, 76), (85, 85), (85, 85),
        (85, 85), (85, 85),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_kiss/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if i >= 3 {
            let idx = i - 3;
            let user_head = images[0].resize_exact(sizes[idx]);
            canvas.draw_image(&user_head, positions[idx], None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 20,
            duration: 0.13,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_kiss",
    nailoong_kiss,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙吻"],
    date_created = local_date(2026, 4, 8),
    date_modified = local_date(2026, 4, 8),
);

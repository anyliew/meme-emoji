use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn maodie_whipped(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (70, 73), (71, 69), (71, 69), (65, 66), (52, 61),
        (41, 65), (38, 68), (38, 69), (44, 72), (50, 71),
        (58, 69), (63, 68), (60, 71), (55, 74), (53, 74),
        (54, 73), (64, 71), (77, 72), (81, 72), (81, 72),
        (73, 71), (58, 70), (33, 70), (31, 71), (40, 73),
        (44, 76), (48, 76), (54, 74),
    ];
    let sizes = [
        (59, 59), (59, 59), (59, 59), (59, 59), (59, 59),
        (59, 59), (59, 59), (59, 59), (59, 59), (59, 59),
        (59, 59), (59, 59), (59, 59), (59, 59), (59, 59),
        (59, 59), (59, 59), (59, 59), (59, 59), (59, 59),
        (59, 59), (59, 59), (59, 59), (59, 59), (59, 59),
        (59, 59), (59, 59), (59, 59),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("maodie_whipped/{i}.png"))?;
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
            frame_num: 28,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "maodie_whipped",
    maodie_whipped,
    min_images = 1,
    max_images = 1,
    keywords = &["耄耋打"],
    date_created = local_date(2026, 7, 13),
    date_modified = local_date(2026, 7, 13),
);

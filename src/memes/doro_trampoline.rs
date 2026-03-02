use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_trampoline(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let params = [
        (31, 94), (30, 96), (33, 68), (33, 66), (29, 65),  // 1-5
        (32, 62), (31, 76), (31, 95), (30, 95), (30, 95),  // 6-10
        (32, 67), (32, 66), (33, 65), (36, 65), (34, 74),  // 11-15
    ];
    let user_head = images[0].image.circle().resize_exact((77, 77));
    let func = |i: usize, _images: Vec<Image>| {
        let frame = load_image(format!("doro_trampoline/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, params[i], None);
        canvas.draw_image(&frame, (0,0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 15,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "doro_trampoline",
    doro_trampoline,
    min_images = 1,
    max_images = 1,
    keywords = &["跳床","蹦床","doro蹦床","桃乐丝蹦床","doro跳床","桃乐丝跳床"],
    date_created = local_date(2025, 8, 10),
    date_modified = local_date(2025, 8, 10),
);

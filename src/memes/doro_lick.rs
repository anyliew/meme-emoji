use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_lick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (103, 73),
        (103, 73),
        (103, 73),
        (103, 73),
        (103, 73), // 11-15
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("doro_lick/{i}.png"))?;
        let user_head = images[0].circle().resize_exact((32, 32));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, params[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.09,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "doro_lick",
    doro_lick,
    min_images = 1,
    max_images = 1,
    keywords = &["桃乐丝舔", "doro舔", "Doro舔", "DORO舔"],
    date_created = local_date(2025, 8, 10),
    date_modified = local_date(2025, 8, 10),
);

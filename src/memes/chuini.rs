use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn chuini(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let params = [
        (56, 50, 23, 142),
        (49, 34, 21, 158),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let (w, h, x, y) = params[i];
        let frame = load_image(format!("chuini/{i}.png"))?;
        let user_head = images[0].square().resize_exact((w,h));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(user_head, (x,y), None);
        canvas.draw_image(frame, (0,0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 2,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "chuini",
    chuini,
    min_images = 1,
    max_images = 1,
    keywords = &["捶你"],
    date_created = local_date(2025, 8, 10),
    date_modified = local_date(2025, 8, 10),
);

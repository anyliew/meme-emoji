use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn whisper(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("whisper/{i}.png"))?;
        let user_head = images[0].resize_exact((130, 130));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, (7,64), None);
        canvas.draw_image(&frame, (0,0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 30,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "whisper",
    whisper,
    min_images = 1,
    max_images = 1,
    keywords = &["窃窃私语"],
    date_created = local_date(2025, 8, 11),
    date_modified = local_date(2025, 9, 8),
);

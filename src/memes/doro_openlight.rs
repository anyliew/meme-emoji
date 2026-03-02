use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_openlight(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [(33, 35); 63];
    let user_head = images[0].image.circle().resize_exact((55, 55));
    let func = |i: usize, _images: Vec<Image>| {
        let frame = load_image(format!("doro_openlight/{i}.png"))?;
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
            frame_num: 63,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "doro_openlight",
    doro_openlight,
    min_images = 1,
    max_images = 1,
    keywords = &["开灯", "桃乐丝开灯", "doro开灯", "Doro开灯", "DORO开灯"],
    date_created = local_date(2025, 9, 12),
    date_modified = local_date(2025, 9, 12),
);

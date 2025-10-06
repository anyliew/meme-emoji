use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn shuai(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(83, 69, 43, 135), (53, 45, 49, 103), (53, 44, 138, 100), (61, 62, 149, 125)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("shuai/{i}.png"))?;
        let (w, h, x, y) = locs[i];
        let image = images[0].resize_bound((w, h), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&image, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 2, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "shuai",
    shuai,
    min_images = 2,
    max_images = 2,
    keywords = &["甩"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

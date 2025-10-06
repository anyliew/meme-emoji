use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn duidi(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs =
        [(92, 75, 22, 6), (93, 76, 20, 6), (92, 76, 23, 10), (92, 76, 22, 5), (94, 77, 23, 2)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("duidi/{i}.png"))?;
        let (w, h, x, y) = locs[i];
        let image = images[0].circle().resize_exact((w, h));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 5, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "duidi",
    duidi,
    min_images = 1,
    max_images = 1,
    keywords = &["怼地", "怼"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

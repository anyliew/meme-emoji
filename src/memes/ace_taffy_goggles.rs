use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ace_taffy_goggles(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(14, 30); 34];
    let sizes = [(191, 191); 34];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("ace_taffy_goggles/{i}.png"))?;
        let head = images[0].square().resize_exact(sizes[i]).circle();
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 34,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "ace_taffy_goggles",
    ace_taffy_goggles,
    min_images = 1,
    max_images = 1,
    keywords = &["护目镜"],
    date_created = local_date(2026, 8, 2),
    date_modified = local_date(2026, 8, 2),
);

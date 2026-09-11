use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_roccia_draw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (131, 265), (128, 262), (129, 261), (127, 256), (128, 250),
        (131, 246), (135, 242), (140, 238), (147, 235), (153, 233),
        (159, 230), (164, 229), (168, 228), (171, 227), (172, 227),
        (172, 226), (172, 225), (172, 225), (172, 224), (172, 224),
        (173, 223), (172, 223), (172, 223), (172, 223), (172, 223),
        (172, 223), (172, 223), (172, 223), (172, 223), (172, 223),
        (172, 223), (172, 223), (172, 223), (172, 223), (172, 223),
        (172, 223), (172, 223), (172, 223), (172, 223), (172, 223),
        (172, 223), (172, 223), (172, 223),
    ];
    let sizes = [
        (43, 35), (49, 38), (50, 39), (59, 44), (65, 50),
        (70, 52), (74, 53), (77, 53), (78, 54), (78, 53),
        (79, 54), (79, 54), (80, 54), (79, 55), (79, 54),
        (79, 55), (79, 55), (79, 54), (79, 55), (79, 54),
        (78, 55), (79, 55), (79, 55), (79, 55), (79, 55),
        (79, 55), (79, 55), (79, 55), (79, 55), (80, 55),
        (79, 55), (79, 55), (79, 55), (79, 55), (79, 55),
        (79, 55), (79, 55), (79, 55), (79, 55), (79, 55),
        (79, 55), (79, 55), (79, 55),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_roccia_draw/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if i >= 31 {
            let j = i - 31;
            let head = images[0].resize_fit(sizes[j], Fit::Cover);
            canvas.draw_image(&head, positions[j], None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 74,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_roccia_draw",
    kurogames_roccia_draw,
    min_images = 1,
    max_images = 1,
    keywords = &["洛可可画画"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 5, 8),
    date_modified = local_date(2026, 5, 8),
);

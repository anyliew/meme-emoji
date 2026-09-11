use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_roccia_love(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (106, 242), (105, 244), (103, 246), (102, 247), (101, 249),
        (100, 250), (99, 251), (98, 251), (98, 251), (97, 250),
        (97, 249), (97, 248), (98, 247), (99, 246), (99, 244),
        (101, 243), (103, 242), (104, 241), (105, 239), (107, 238),
        (108, 237), (109, 236), (110, 235), (110, 235), (111, 234),
        (111, 234), (111, 235), (111, 236), (110, 236), (109, 237),
        (108, 238), (107, 240), (106, 242),
    ];
    let sizes = [
        (94, 57), (94, 56), (95, 54), (95, 53), (95, 51),
        (95, 50), (95, 49), (95, 49), (95, 49), (96, 50),
        (96, 51), (97, 51), (96, 51), (96, 52), (97, 54),
        (95, 54), (94, 54), (95, 55), (95, 57), (93, 57),
        (94, 58), (94, 59), (93, 60), (94, 60), (93, 61),
        (93, 61), (93, 61), (92, 60), (93, 60), (93, 60),
        (94, 59), (94, 58), (94, 57),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_roccia_love/{i}.png"))?;
        let user_head = images[0].resize_fit(sizes[i], Fit::Cover);
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
            frame_num: 33,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_roccia_love",
    kurogames_roccia_love,
    min_images = 1,
    max_images = 1,
    keywords = &["洛可可喜欢"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 5, 8),
    date_modified = local_date(2026, 5, 8),
);

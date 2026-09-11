use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_sigrika_love(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (126, 388), (128, 393), (129, 405), (127, 420), (123, 433),
        (119, 438), (115, 437), (112, 433), (115, 427), (124, 418),
        (129, 405), (129, 391), (129, 387), (129, 396), (129, 414),
        (126, 438), (120, 439), (115, 437), (108, 430), (112, 413),
    ];
    let sizes = [
        (253, 112), (249, 107), (247, 95), (251, 80), (259, 67),
        (266, 62), (274, 63), (279, 67), (274, 73), (257, 82),
        (247, 95), (247, 109), (247, 113), (247, 104), (248, 86),
        (253, 62), (264, 61), (274, 63), (286, 70), (279, 87),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_sigrika_love/{i}.png"))?;
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
            frame_num: 20,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_sigrika_love",
    kurogames_sigrika_love,
    min_images = 1,
    max_images = 1,
    keywords = &["西格莉卡喜欢", "耙耙柑喜欢"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 5, 8),
    date_modified = local_date(2026, 5, 8),
);

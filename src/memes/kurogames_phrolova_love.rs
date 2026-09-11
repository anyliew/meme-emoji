use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_phrolova_love(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (76, 233), (77, 236), (78, 243), (77, 252), (74, 260),
        (72, 263), (69, 263), (69, 261), (70, 256), (76, 252),
        (79, 244), (78, 235), (78, 233), (78, 238), (77, 249),
        (76, 263), (72, 264), (69, 263), (66, 259), (70, 251),
    ];
    let sizes = [(160, 70); 20];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_phrolova_love/{i}.png"))?;
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
    "kurogames_phrolova_love",
    kurogames_phrolova_love,
    min_images = 1,
    max_images = 1,
    keywords = &["弗洛洛喜欢"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 6, 10),
    date_modified = local_date(2026, 6, 10),
);

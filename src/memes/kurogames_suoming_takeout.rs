use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_suoming_takeout(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(51, 180); 19];
    let sizes = [(115, 115); 19];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_suoming_takeout/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if i >= 17 {
            let idx = i - 17;
            let head = images[0].square().resize_exact(sizes[idx]);
            canvas.draw_image(&head, positions[idx], None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 36,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_suoming_takeout",
    kurogames_suoming_takeout,
    min_images = 1,
    max_images = 1,
    keywords = &["锁暝掏"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 7, 22),
    date_modified = local_date(2026, 7, 24),
);

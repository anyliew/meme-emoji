use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_aemeath_love(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (35, 127), (37, 127), (37, 126), (38, 126), (36, 128),
        (38, 125),
    ];
    let sizes = [
        (91, 35), (90, 35), (91, 36), (90, 36), (93, 34),
        (90, 37),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_aemeath_love/{i}.png"))?;
        let head = images[0].square().resize_fit(sizes[i], Fit::Cover);
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
            frame_num: 6,
            duration: 0.3,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_aemeath_love",
    kurogames_aemeath_love,
    min_images = 1,
    max_images = 1,
    keywords = &["爱弥斯爱心"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 8, 24),
    date_modified = local_date(2026, 8, 24),
);

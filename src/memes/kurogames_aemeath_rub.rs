use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_aemeath_rub(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (181, 139), (181, 138), (181, 134), (181, 129), (181, 126),
        (181, 126), (181, 127), (181, 131), (181, 136), (181, 139),
        (181, 138), (181, 134), (181, 130), (181, 127), (181, 124),
        (181, 127), (181, 131), (181, 136),
    ];
    let sizes = [
        (121, 104), (121, 103), (121, 103), (121, 103), (121, 101),
        (121, 100), (121, 103), (121, 104), (121, 103), (121, 104),
        (121, 103), (121, 103), (121, 101), (121, 100), (121, 103),
        (121, 103), (121, 104), (121, 103),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_aemeath_rub/{i}.png"))?;
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
            frame_num: 18,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_aemeath_rub",
    kurogames_aemeath_rub,
    min_images = 1,
    max_images = 1,
    keywords = &["爱弥斯搓"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 8, 24),
    date_modified = local_date(2026, 8, 24),
);

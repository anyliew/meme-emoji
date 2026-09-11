use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_denia_lick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (45, 299), (45, 297), (44, 295), (43, 293), (41, 292),
        (39, 292), (37, 291), (35, 292), (34, 293), (31, 296),
        (30, 299), (30, 301), (29, 304), (31, 306), (31, 308),
        (32, 310), (33, 311), (35, 311), (37, 311), (39, 311),
        (41, 309), (43, 306), (45, 304), (45, 302),
    ];
    let sizes = [
        (153, 153), (153, 153), (153, 153), (153, 153), (153, 153),
        (153, 153), (153, 153), (153, 153), (153, 153), (153, 153),
        (153, 153), (153, 153), (153, 153), (153, 153), (153, 153),
        (153, 153), (153, 153), (153, 153), (153, 153), (153, 153),
        (153, 153), (153, 153), (153, 153), (153, 153),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_denia_lick/{i}.png"))?;
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
            frame_num: 24,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_denia_lick",
    kurogames_denia_lick,
    min_images = 1,
    max_images = 1,
    keywords = &["达妮娅舔"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 6, 19),
    date_modified = local_date(2026, 6, 19),
);

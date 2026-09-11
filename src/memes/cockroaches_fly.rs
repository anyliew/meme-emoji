use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn cockroaches_fly(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (121, 307), (121, 307), (121, 307), (121, 306), (121, 306),
        (120, 304), (120, 304), (119, 302), (119, 302), (119, 298),
        (119, 295), (118, 294), (118, 291), (118, 290), (127, 284),
        (136, 278), (146, 272), (156, 265), (166, 262), (172, 257),
        (183, 250), (192, 245), (203, 237), (203, 233), (203, 232),
        (204, 229), (204, 228), (198, 226), (197, 222), (197, 220),
        (191, 216), (191, 215), (182, 203), (182, 203), (174, 190),
        (170, 189), (162, 176), (150, 164), (150, 164), (145, 150),
        (145, 150), (135, 135), (135, 135), (124, 125), (123, 123),
        (123, 115), (115, 108), (116, 108), (116, 99), (116, 99),
        (87, 71), (87, 71),
    ];
    let sizes = [
        (4, 4), (4, 4), (4, 4), (5, 5), (5, 5),
        (6, 6), (6, 6), (9, 9), (9, 9), (9, 9),
        (9, 9), (10, 10), (11, 11), (11, 11), (11, 11),
        (15, 15), (15, 15), (15, 15), (15, 15), (23, 23),
        (23, 23), (23, 23), (23, 23), (30, 30), (30, 30),
        (30, 30), (30, 30), (41, 41), (46, 46), (46, 46),
        (61, 61), (61, 61), (82, 82), (82, 82), (97, 97),
        (105, 105), (123, 123), (146, 146), (146, 146), (159, 159),
        (159, 159), (181, 181), (181, 181), (203, 203), (203, 203),
        (203, 203), (220, 220), (220, 220), (220, 220), (220, 220),
        (277, 277), (277, 277),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("cockroaches_fly/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if (7..=58).contains(&i) {
            let idx = i - 7;
            let user_head = images[0].resize_exact(sizes[idx]);
            canvas.draw_image(&user_head, positions[idx], None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 61,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "cockroaches_fly",
    cockroaches_fly,
    min_images = 1,
    max_images = 1,
    keywords = &["飞天小强", "飞天蟑螂"],
    date_created = local_date(2026, 6, 10),
    date_modified = local_date(2026, 6, 10),
);

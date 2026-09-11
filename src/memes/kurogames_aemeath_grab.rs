use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_aemeath_grab(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (9, 91), (9, 91), (9, 91), (8, 91), (8, 91),
        (8, 91), (8, 91), (8, 91), (8, 91), (8, 91),
        (8, 91), (8, 91), (9, 91), (8, 91), (12, 130),
        (19, 153), (19, 159), (19, 156), (18, 169), (19, 181),
        (19, 172), (19, 161), (19, 165), (19, 169), (19, 169),
        (19, 169), (19, 169), (19, 169), (19, 169), (19, 169),
        (19, 169), (19, 169), (19, 169), (19, 169), (19, 169),
        (19, 169),
    ];
    let sizes = [
        (244, 209), (244, 209), (244, 209), (245, 209), (245, 209),
        (245, 209), (246, 209), (245, 209), (245, 209), (245, 209),
        (245, 209), (246, 209), (245, 209), (245, 209), (188, 170),
        (145, 147), (133, 141), (127, 144), (125, 131), (124, 119),
        (124, 128), (124, 139), (124, 135), (125, 131), (125, 131),
        (125, 131), (125, 131), (125, 131), (127, 131), (125, 131),
        (125, 131), (125, 131), (125, 131), (125, 131), (125, 131),
        (125, 131),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_aemeath_grab/{i}.png"))?;
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
            frame_num: 36,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_aemeath_grab",
    kurogames_aemeath_grab,
    min_images = 1,
    max_images = 1,
    keywords = &["爱弥斯抓"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 6, 10),
    date_modified = local_date(2026, 6, 10),
);

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_camellya(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(40, 50); 15];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_camellya/{i}.png"))?;
        let user_head = images[0].circle().resize_exact((111, 111));
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
            frame_num: 15,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_camellya",
    kurogames_camellya,
    min_images = 1,
    max_images = 1,
    keywords = &["椿"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 6, 29),
    date_modified = local_date(2026, 6, 29),
);

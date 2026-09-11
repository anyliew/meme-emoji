use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_songlun_touch(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(142, 0); 37];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_songlun_touch/{i}.png"))?;
        let user_head = images[0].resize_exact((242, 242));
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
            frame_num: 37,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_songlun_touch",
    kurogames_songlun_touch,
    min_images = 1,
    max_images = 1,
    keywords = &["松伦摸"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 4, 8),
    date_modified = local_date(2026, 4, 8),
);

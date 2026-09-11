use skia_safe::Image;
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};
use crate::{options::NoOptions, register_meme};

fn ikun_year(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(27, 35); 12];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("ikun_year/{}.png", i % 7))?;
        let user_head = images[0].resize_exact((172, 172));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 12, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "ikun_year",
    ikun_year,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["坤年"],
    date_created = local_date(2026, 3, 30),
    date_modified = local_date(2026, 3, 30),
}

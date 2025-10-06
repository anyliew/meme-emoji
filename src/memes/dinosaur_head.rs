use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn dinosaur_head(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("dinosaur_head/0.png")?;
    let mut surface = frame.to_surface();
    let _canvas = surface.canvas();
    // 删除了绘制文本的代码
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_exact((620, 620));
        canvas.draw_image(&img, (215, 245), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "dinosaur_head",
    dinosaur_head,
    min_images = 1,
    max_images = 1,
    keywords = &["恐龙头"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn something(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("something/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：圆形裁剪并调整尺寸
        let img = images[0]
            .resize_fit((93, 93), Fit::Cover)
            .circle();
        canvas.draw_image(&img, (138, 207), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "something",
    something,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["什么东西"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
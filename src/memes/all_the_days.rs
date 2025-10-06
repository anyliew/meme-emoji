use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn all_the_days(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("all_the_days/0.png")?;

    let func = |images: Vec<Image>| {
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制背景帧（在最底层）
    canvas.draw_image(&frame, (0, 0), None);
    
    // 处理图片
    let image1 = images[0].circle().resize_fit((95, 95), Fit::Cover).circle();
    let image2 = images[1].circle().resize_fit((95, 95), Fit::Cover).circle();
    
    // 最后绘制图片（在最高层）
    canvas.draw_image(&image1, (255, 255), None);
    canvas.draw_image(&image2, (425, 210), None);
    
    Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "all_the_days",
    all_the_days,
    min_images = 2,
    max_images = 2,
    keywords = &["一生一世"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

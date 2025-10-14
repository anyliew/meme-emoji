use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ly01(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("ly01/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 处理第二张图片 (images[1]) - 位置 (933, 251)，大小 235x235
        let img1 = images[1].resize_fit((235, 235), Fit::Cover);
        canvas.draw_image(&img1, (933, 251), None);
        
        // 处理第一张图片 (images[0]) - 位置 (272, 243)，大小 235x235
        let img0 = images[0].resize_fit((235, 235), Fit::Cover);
        canvas.draw_image(&img0, (272, 243), None);
        
        // 最后绘制背景帧
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "ly01",
    ly01,
    min_images = 2,
    max_images = 2,
    keywords = &["ly01", "ly-1", "LY-1"],
    date_created = local_date(2025, 9, 4),
    date_modified = local_date(2025, 9, 21),
);
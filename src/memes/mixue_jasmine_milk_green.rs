use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mixue_jasmine_milk_green(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mixue_jasmine_milk_green/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 第一个头像尺寸: 100x85（矩形，没有圆形裁剪）
        let img1 = images[1].resize_fit((100, 85), Fit::Cover);
        
        // 第二个头像尺寸: 100x85（矩形，没有圆形裁剪）
        let img2 = images[0].resize_fit((100, 85), Fit::Cover);
        
        // 绘制第一个头像，坐标(218, 458)
        canvas.draw_image(&img1, (218, 458), None);
        // 绘制第二个头像，坐标(452, 395)
        canvas.draw_image(&img2, (452, 395), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mixue_jasmine_milk_green",
    mixue_jasmine_milk_green,
    min_images = 2,
    max_images = 2,
    min_texts = 0,
    max_texts = 0,
    keywords = &["茉莉奶绿"],
    date_created = local_date(2025, 6, 20),
    date_modified = local_date(2025, 6, 20),
);
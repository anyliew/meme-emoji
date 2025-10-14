use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn throwing_poop(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("throwing_poop/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 第一个头像尺寸: 125x125（没有圆形裁剪）
        let img1 = images[0].resize_fit((125, 125), Fit::Cover);
        
        // 第二个头像尺寸: 125x125（没有圆形裁剪）
        let img2 = images[0].resize_fit((125, 125), Fit::Cover);
        // 创建旋转后的第二个头像（Rust中旋转方向相反，所以用10度）
        let rotated_img2 = img2.rotate(10.0);
        
        // 第三个头像尺寸: 108x108（没有圆形裁剪）
        let img3 = images[0].resize_fit((108, 108), Fit::Cover);
        // 创建旋转后的第三个头像（Rust中旋转方向相反，所以用-20度）
        let rotated_img3 = img3.rotate(-20.0);
        
        // 绘制第一个头像，坐标(347, 201)
        canvas.draw_image(&img1, (347, 201), None);
        // 绘制第二个头像，坐标(310, 553)
        canvas.draw_image(&rotated_img2, (310, 553), None);
        // 绘制第三个头像，坐标(297, 1003)
        canvas.draw_image(&rotated_img3, (297, 1003), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "throwing_poop",
    throwing_poop,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["扔史"],
    date_created = local_date(2025, 9, 21),
    date_modified = local_date(2025, 9, 23),
);
use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn shipborne_laser_weapons(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("shipborne_laser_weapons/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 第一个头像尺寸: 167x167（没有圆形裁剪）
        let img1 = images[0].resize_fit((167, 167), Fit::Cover);
        
        // 第二个头像尺寸: 73x73（较小，没有圆形裁剪）
        let img2 = images[0].resize_fit((73, 73), Fit::Cover);
        
        // 绘制第一个头像，坐标(118, 212)
        canvas.draw_image(&img1, (118, 212), None);
        // 绘制第二个头像，坐标(572, 918)
        canvas.draw_image(&img2, (572, 918), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "shipborne_laser_weapons",
    shipborne_laser_weapons,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["舰载激光武器"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
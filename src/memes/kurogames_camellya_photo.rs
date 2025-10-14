use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_camellya_photo(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_camellya_photo/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 头像尺寸: 345x310，旋转5度（Rust中旋转方向相反，所以用正数），坐标(170, 500)
        let img = images[0].resize_fit((345, 310), Fit::Cover);
        
        // 创建旋转后的图像（Rust中旋转方向相反，所以用正数5度）
        let rotated_img = img.rotate(5.0);
        
        // 绘制旋转后的头像
        canvas.draw_image(&rotated_img, (170, 500), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_camellya_photo",
    kurogames_camellya_photo,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["大傻椿照片", "傻椿照片"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
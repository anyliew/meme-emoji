use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nakano_ltsuki(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("nakano_ltsuki/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 头像尺寸: 195x195（正方形，没有圆形裁剪）
        let img = images[0].resize_fit((195, 195), Fit::Cover);
        
        // 绘制头像，坐标(125, 130)
        canvas.draw_image(&img, (125, 130), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "nakano_ltsuki",
    nakano_ltsuki,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["中野五月"],
    date_created = local_date(2025, 7, 2),
    date_modified = local_date(2025, 7, 2),
);
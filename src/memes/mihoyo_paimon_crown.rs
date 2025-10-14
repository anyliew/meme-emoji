use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_paimon_crown(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_paimon_crown/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 头像尺寸: 490x490，圆形裁剪（尺寸较大）
        let img = images[0]
            .resize_fit((490, 490), Fit::Cover)
            .circle();
        
        // 绘制圆形头像，坐标(72, 84)
        canvas.draw_image(&img, (72, 84), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_paimon_crown",
    mihoyo_paimon_crown,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["派蒙王冠", "派蒙皇冠", "👑"],
    date_created = local_date(2025, 6, 1),
    date_modified = local_date(2025, 6, 1),
);
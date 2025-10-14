use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn scissor_seven_head(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("scissor_seven_head/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 头像尺寸: 200x200（正方形，没有圆形裁剪）
        let img = images[0].resize_fit((200, 200), Fit::Cover);
        
        // 创建旋转后的图像（Rust中旋转方向相反，所以用4度）
        let rotated_img = img.rotate(4.0);
        
        // 绘制旋转后的头像，坐标(100, 120)
        canvas.draw_image(&rotated_img, (100, 120), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "scissor_seven_head",
    scissor_seven_head,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["伍六七", "阿七"],
    date_created = local_date(2025, 7, 2),
    date_modified = local_date(2025, 7, 2),
);
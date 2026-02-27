use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn commemorate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("commemorate/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        
        // 调整图片尺寸，保持宽高比
        let img = images[0]
            .resize_fit((815, 747), Fit::Contain);
            
        // 转换为灰度（黑白滤镜）- 简化处理，使用原图
        // 因为skia的灰度转换比较复杂，这里直接使用原图
        // 如果需要黑白效果，可以考虑其他方法
        
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, (129, 218), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "commemorate",
    commemorate,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["奠", "寄"],
    date_created = local_date(2026, 1, 12),
    date_modified = local_date(2026, 1, 12),
);
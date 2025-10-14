use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_hilichurl_frame(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_hilichurl_frame/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        // 头像尺寸: 240x240，圆形裁剪
        let img = images[0]
            .resize_fit((240, 240), Fit::Cover)
            .circle();
        
        // 绘制圆形头像，坐标(40, 40)
        canvas.draw_image(&img, (40, 40), None);
        // 绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_hilichurl_frame",
    mihoyo_hilichurl_frame,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["丘丘人相框"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
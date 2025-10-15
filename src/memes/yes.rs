use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn yes(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("yes/0.png")?;
    
    let name = &images[0].name;
    
    let text = format!("{name} YES!👍🏻👍🏻");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：调整尺寸（没有圆形裁剪）
        let img = images[0]
            .resize_fit((165, 165), Fit::Cover);
        canvas.draw_image(&img, (396, 452), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(295, 817, 980, 936),
            &text,
            15.0,
            120.0,
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "yes",
    yes,
    min_images = 1,
    max_images = 1,
    keywords = &["yes", "Yes", "YES"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
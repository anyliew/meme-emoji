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

fn yuzu_soft_ticket(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("yuzu_soft_ticket/0.png")?;
    
    let ta = "柚子厨";
    let mut name = ta.to_string();
    if !texts.is_empty() {
        name = texts[0].clone();
    }
    
    let text = format!("{name}");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：调整尺寸（没有圆形裁剪）
        let img = images[0]
            .resize_fit((300, 300), Fit::Cover);
        canvas.draw_image(&img, (72, 330), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(485, 544, 625, 583),
            &text,
            30.0,
            100.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "yuzu_soft_ticket",
    yuzu_soft_ticket,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["准考证"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
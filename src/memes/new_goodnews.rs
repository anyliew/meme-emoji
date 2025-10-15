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

fn new_goodnews(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("new_goodnews/0.png")?;
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else {
        "她".to_string()
    };

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        let img = &images[0];
        
        // 先绘制用户图片在最底层
        let user_img = img.resize_bound((300, 300), Fit::Cover);
        canvas.draw_image(&user_img, (210, 270), None);
        
        // 然后绘制frame覆盖在用户图片之上
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制第一段文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(235, 615, 478, 659),
            &name,
            10.0,  // min_fontsize
            100.0, // max_fontsize
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 223, 0))
            ),
        )?;
        
        // 绘制第二段文字（如果有）
        if texts.len() > 1 {
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(199, 708, 512, 923),
                &texts[1],
                10.0,  // min_fontsize
                60.0,  // max_fontsize
                text_params!(
                    font_families = &["FZXS14"],
                    text_align = TextAlign::Center,
                    paint = new_paint(Color::from_rgb(255, 215, 0))
                ),
            )?;
        }
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "new_goodnews",
    new_goodnews,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 2,
    keywords = &["新喜报"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2025, 10, 3),
);
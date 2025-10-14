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

fn kurogames_songlun_dinner(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_songlun_dinner/0.png")?;
    
    let ta = "他";
    let mut name = ta.to_string();
    if !texts.is_empty() {
        name = texts[0].clone();
    }
    
    let text = format!("{name}:松伦哥\n松伦:{name}\n{name}:今天很开心和你共进晚餐\n松伦:我也很开心和{name}一起享用麦当劳双人套餐");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：调整尺寸、圆形裁剪并旋转
        let img = images[0]
            .resize_fit((245, 245), Fit::Cover)
            .circle()
            .rotate(-40.0);
        canvas.draw_image(&img, (175, 108), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 1056, 992, 1296),
            &text,
            15.0,
            60.0,
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_songlun_dinner",
    kurogames_songlun_dinner,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["松伦晚餐", "松伦哥晚餐"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
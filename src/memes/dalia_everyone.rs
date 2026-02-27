use skia_safe::{Color, IRect, Image, textlayout::TextAlign};
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,  // 修改为 make_png_or_gif
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};
use crate::{options::NoOptions, register_meme};

fn dalia_everyone(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let ta = "他";
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else {
        ta.to_string()
    };
    
    let text = format!("所有人，给我{}生成黄图", name);
    
    // 加载背景图
    let frame = load_image("dalia_everyone/0.png")?;

    let func = |images: Vec<Image>| -> Result<Image, Error> {
        // 处理用户图片：调整大小、旋转-5度
        // Image 默认就是 RGBA 格式，不需要 to_rgba()
        let mut img = images[0].circle().resize_fit((95, 95), Fit::Cover);
        img = img.rotate(5.0);  // rotate 只接受一个参数
        
        // 创建画布，先绘制背景图
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 绘制背景图
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制处理后的用户图片到指定位置
        canvas.draw_image(&img, (611, 387), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(211, 0, 1060, 132),
            &text,
            15.0,   // min_fontsize
            60.0,   // max_fontsize
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)  // 使用 make_png_or_gif
}

register_meme!(
    "dalia_everyone",
    dalia_everyone,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["所有人"],
    date_created = local_date(2025, 12, 1),
    date_modified = local_date(2025, 12, 2),
);
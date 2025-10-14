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

fn qixi_festival(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("qixi_festival/0.png")?;
    
    let ta = "他";
    let mut name = ta.to_string();
    if !texts.is_empty() {
        name = texts[0].clone();
    }
    
    let text = format!("农历七月初七 新历8月29日 20:00 \n{name}和派蒙在原神官方直播间过七夕");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：圆形裁剪并调整尺寸
        let img = images[0]
            .resize_fit((370, 370), Fit::Cover)
            .circle();
        canvas.draw_image(&img, (618, 284), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(58, 1085, 1330, 1234),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "qixi_festival",
    qixi_festival,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["七夕和谁过", "七夕和谁过?", "七夕和谁过？"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
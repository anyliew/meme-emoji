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

fn jd_delivery_person(images: Vec<InputImage>, _texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("jd_delivery_person/0.png")?;
    let name = images[0].name.clone(); // 克隆name而不是借用
    
    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        let img = &images[0];
        
        // 先绘制用户图片在最底层
        let darkened_img = img.resize_bound((630, 630), Fit::Cover);
        let user_img = darkened_img.rotate(20.0);
        canvas.draw_image(&user_img, (420, 1000), None);
        
        // 然后绘制frame覆盖在用户图片之上
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制旋转文本在最上层
        // 保存当前画布状态
        canvas.save();
        
        // 移动到文本位置并旋转
        canvas.translate((653.0, 1710.0));
        canvas.rotate(17.0, None);
        
        // 绘制文本
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 0, 930 - 353, 2104 - 1710), // 文本框大小
            &name, // 使用克隆的name
            45.0,  // min_fontsize
            60.0,  // max_fontsize
            text_params!(
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(58, 60, 73))
            ),
        )?;
        
        // 恢复画布状态
        canvas.restore();
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "jd_delivery_person",
    jd_delivery_person,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["京东外卖骑手", "京东外卖工牌"],
    date_created = local_date(2025, 3, 24),
    date_modified = local_date(2025, 9, 25),
);
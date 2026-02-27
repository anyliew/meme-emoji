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

fn cleverer(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 处理文字：根据用户信息或输入文本来确定名称
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else {
        // 注意：Rust 版本可能需要从用户信息中获取名称
        // 这里简化为默认文本
        "这家伙".to_string()
    };
    
    let text = format!("你还能有{}聪明？", name);
    
    // 加载背景图
    let frame = load_image("cleverer/0.png")?;

    let func = |images: Vec<Image>| -> Result<Image, Error> {
        // 获取输入图片
        let input_img = &images[0];
        
        // 定义目标尺寸和位置
        let screen_x = -60;
        let screen_y = 99;
        let target_width = 680;
        let target_height = 680;
        
        // 计算图片适应区域的缩放和裁剪
        let target_ar = target_width as f32 / target_height as f32;
        let img_ar = input_img.width() as f32 / input_img.height() as f32;
        
        let final_img = if img_ar > target_ar {
            // 图片更宽：按高度缩放，水平裁剪
            let new_height = target_height;
            let new_width = (new_height as f32 * img_ar) as i32;
            // 使用 resize_fit 进行缩放
            let resized = input_img.resize_fit((new_width, new_height), Fit::Cover);
            let left = (new_width - target_width) / 2;
            let right = left + target_width;
            // crop 方法直接返回裁剪后的图片，不需要 ?
            resized.crop(IRect::from_ltrb(left, 0, right, new_height))
        } else {
            // 图片更高：按宽度缩放，垂直裁剪
            let new_width = target_width;
            let new_height = (new_width as f32 / img_ar) as i32;
            // 使用 resize_fit 进行缩放
            let resized = input_img.resize_fit((new_width, new_height), Fit::Cover);
            let top = (new_height - target_height) / 2;
            let bottom = top + target_height;
            // crop 方法直接返回裁剪后的图片，不需要 ?
            resized.crop(IRect::from_ltrb(0, top, new_width, bottom))
        };
        
        // 创建画布，绘制背景图作为最底层
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制处理后的用户图片到指定位置
        canvas.draw_image(&final_img, (screen_x, screen_y), None);
        
        // 然后绘制frame作为上层
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字在最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(40, 845, 1056, 992),
            &text,
            20.0,   // min_fontsize
            100.0,  // max_fontsize
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0))  // 文字颜色黑色
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "cleverer",
    cleverer,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["比聪明"],
    date_created = local_date(2025, 11, 21),
    date_modified = local_date(2025, 11, 21),
);
use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn yuzu_soft_shocked(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name},你是柚...柚子厨?!");
    let frame = load_image("yuzu_soft_shocked/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制背景框架
    canvas.draw_image(&frame, (0, 0), None);
    
    // 然后绘制文字（在最上层）
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(34, 100, 257, 133),
        &text,
        20.0,
        100.0,
        None,
    )?;
    
    let frame_with_text = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame_with_text.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户头像
        let img = images[0].circle().resize_exact((33, 33));
        
        // 先绘制带文字的框架
        canvas.draw_image(&frame_with_text, (0, 0), None);
        
        // 最后绘制用户头像（在最上层）
        canvas.draw_image(&img, (0, 100), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "yuzu_soft_shocked",
    yuzu_soft_shocked,
    tags = MemeTags::yuzu_soft(),
    min_images = 1,
    max_images = 1,
    keywords = &["震惊柚子厨"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
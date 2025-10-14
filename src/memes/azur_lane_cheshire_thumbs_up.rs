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

fn azur_lane_cheshire_thumbs_up(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("azur_lane_cheshire_thumbs_up/0.jpg")?;
    
    let ta = "他";
    let mut name = ta.to_string();
    if !texts.is_empty() {
        name = texts[0].clone();
    }
    
    let text = format!("{name},你真是一个大聪明\n柴郡为你点个赞👍🏻");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：圆形裁剪并调整尺寸
        let img = images[0]
            .resize_fit((230, 230), Fit::Cover)
            .circle();
        canvas.draw_image(&img, (30, 120), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(207, 1, 739, 150),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "azur_lane_cheshire_thumbs_up",
    azur_lane_cheshire_thumbs_up,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["柴郡点赞", "柴郡猫点赞"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
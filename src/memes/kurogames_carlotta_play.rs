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

fn kurogames_carlotta_play(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let ta = "他";
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else {
        ta.to_string()
    };
    
    let text = format!("无语,拿着{}\n一边玩去吧", name);
    
    let frame = load_image("kurogames_carlotta_play/0.png")?;

    let func = |images: Vec<Image>| -> Result<Image, Error> {
        let img = images[0]
            .resize_fit((290, 340), Fit::Cover)
            .rotate(60.0);  // Python -60° 对应 Rust 60°
        
        // 先创建背景图的surface
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 由于 below=True，需要先绘制用户图片作为底层
        canvas.draw_image(&img, (1090, 920), None);
        // 然后绘制frame在上层
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字在最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 1350, 1672, 1726),
            &text,
            15.0,
            1500.0,
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
    "kurogames_carlotta_play",
    kurogames_carlotta_play,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["一边玩去吧", "一边玩去"],
    date_created = local_date(2025, 10, 11),
    date_modified = local_date(2025, 10, 11),
);
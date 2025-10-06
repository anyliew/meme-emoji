use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn hitachi_mako_together(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name}：我要和四季夏目永远在一起");
    let frame = load_image("hitachi_mako_together/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(170, 20, 900, 100),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(57, 49, 46))
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制背景和文字
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制用户图片（在最上层）
        let img = images[0].circle().resize_exact((180, 180));
        canvas.draw_image(&img, (665, 180), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "hitachi_mako_together",
    hitachi_mako_together,
    min_images = 1,
    max_images = 1,
    keywords = &["和她在一起"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
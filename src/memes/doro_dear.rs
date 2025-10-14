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

fn doro_dear(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("doro_dear/0.png")?;
    
    let name = &images[0].name;
    
    let text = format!("桃乐丝:{name},你永远是桃乐丝的最爱之人❤️");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：调整尺寸（没有圆形裁剪）
        let img = images[0]
            .resize_fit((100, 100), Fit::Cover);
        canvas.draw_image(&img, (576, 529), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 675, 1200, 812),
            &text,
            15.0,
            60.0,
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
    "doro_dear",
    doro_dear,
    min_images = 1,
    max_images = 1,
    keywords = &["最爱", "doro最爱", "Doro最爱", "DORO最爱", "桃乐丝最爱"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
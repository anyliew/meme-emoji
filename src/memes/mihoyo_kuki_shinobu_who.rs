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

fn mihoyo_kuki_shinobu_who(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_kuki_shinobu_who/0.png")?;
    
    let name = &images[0].name;
    
    let text = format!("\n久岐忍:此人是谁?\n\n旅行者:此人是{name},擅长跳、唱、rap、打篮球\n\n旅行者:是稻妻国两年半不可多得的尤物~\n");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：调整尺寸并旋转
        let img = images[0]
            .resize_fit((210, 300), Fit::Cover)
            .rotate(-11.0);
        canvas.draw_image(&img, (932, 310), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 1023, 2048, 1324),
            &text,
            15.0,
            120.0,
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
    "mihoyo_kuki_shinobu_who",
    mihoyo_kuki_shinobu_who,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["此人是谁", "是谁", "是谁？", "是谁？"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
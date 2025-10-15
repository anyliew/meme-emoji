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

fn widow(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("widow/0.png")?;
    
    let name = &images[0].name;
    
    let text = format!("祭祀者:安心去吧,{name}桑[さん]\n你最❤️愛の妻子[未亡人]我会好好照顾的\n{name}你去了冥界也要保佑我们");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：调整尺寸、转换为灰度并旋转
        let img = images[0]
            .resize_fit((210, 215), Fit::Cover)
            .grayscale()
            .rotate(-21.0);
        canvas.draw_image(&img, (162, 200), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(3, 877, 925, 1030),
            &text,
            15.0,
            60.0,
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
    "widow",
    widow,
    min_images = 1,
    max_images = 1,
    keywords = &["未亡人"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
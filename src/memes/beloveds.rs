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

fn beloveds(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("だいじなひと{name}👩‍❤️‍💋‍👨");
    let frame = load_image("beloveds/0.png")?;

    let func = |images: Vec<Image>| {
        // 创建与背景图片相同尺寸的表面
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 第一步：绘制用户图片（底层）
        let img = images[0].resize_bound((330, 330), Fit::Cover);
        canvas.draw_image(&img, (330, 140), None);
        
        // 第二步：绘制背景图片（中层）
        canvas.draw_image(&frame, (0, 0), None);
        
        // 第三步：绘制文字（最上层）
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 614, 638, 761),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZKaTong-M19S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "beloveds",
    beloveds,
    min_images = 1,
    max_images = 1,
    keywords = &["挚爱"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
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

fn allegiance(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name}:{name}向你敬礼,忠诚!\n{name}:{name}당신에게 경의를 표합니다,충성!");
    let frame = load_image("allegiance/0.png")?;

    let func = |images: Vec<Image>| {
        // 创建新的surface，先绘制用户图片作为最底层
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制用户图片作为最底层
        let img = images[0].resize_bound((436, 400), Fit::Cover);
        canvas.draw_image(&img, (279, 159), None);
        
        // 然后绘制frame作为中层（覆盖在用户图片之上）
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字作为最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 774, 1061, 942),
            &text,
            20.0,
            100.0,
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
    "allegiance",
    allegiance,
    min_images = 1,
    max_images = 1,
    keywords = &["충성","忠橙","敬礼"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
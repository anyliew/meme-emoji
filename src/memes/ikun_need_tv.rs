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

fn ikun_need_tv(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("坤坤好喜欢{name}\n想要一个{name}❤️~");
    let frame = load_image("ikun_need_tv/0.png")?;

    let func = |images: Vec<Image>| {
        // 创建与框架相同尺寸的表面
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制用户图片作为底层
        let img = images[0].resize_bound((518, 488), Fit::Cover);
        canvas.draw_image(&img, (614, 118), None);
        
        // 然后在上面绘制框架
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字（在最顶层）
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(2, 2, 572, 250),
            &text,
            5.0,
            50.0,
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
    "ikun_need_tv",
    ikun_need_tv,
    min_images = 1,
    max_images = 1,
    keywords = &["坤坤想要"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
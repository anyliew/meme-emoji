use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn yuzu_soft_ayachi_nene(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name},这是你的照片吗？");
    let frame = load_image("yuzu_soft_ayachi_nene/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        // 先绘制所有图像内容
        let image = images[0].resize_bound((165, 165), Fit::Cover).rotate(45.0);
        canvas.draw_image(&image, (500, 410), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字，确保文字在最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 0, 716, 128),
            &text,
            20.0,
            150.0,
            text_params!(
                font_families = &["FZSJ-QINGCRJ"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(57, 49, 46))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "yuzu_soft_ayachi_nene",
    yuzu_soft_ayachi_nene,
    tags = MemeTags::ayachi(),
    min_images = 1,
    max_images = 1,
    keywords = &["宁宁困惑", "绫地宁宁困惑"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);
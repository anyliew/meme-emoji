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

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn yuzu_soft_murasame_clothes(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name},你好变态,穿我的衣物");
    let frame = load_image("yuzu_soft_murasame_clothes/0.jpg")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 130, 339),
        &text,
        20.0,
        100.0,
        text_params!(
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
            font_families = &["FZKaTong-M19S"],
        ),
    )?;

    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_bound((140, 140), Fit::Cover);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&image, (700, 80), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "yuzu_soft_murasame_clothes",
    yuzu_soft_murasame_clothes,
    tags = MemeTags::murasame(),
    min_images = 1,
    max_images = 1,
    keywords = &["丛雨衣服", "丛雨衣物"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

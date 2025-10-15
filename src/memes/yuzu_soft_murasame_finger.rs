use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn yuzu_soft_murasame_finger(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("原来{}是柚子厨", name);
    let frame = load_image("yuzu_soft_murasame_finger/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制文字
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(805, 94, 1728, 274),
        &text,
        20.0,
        150.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    // 然后在同一个 canvas 上绘制图片
    let img = images[0].image.circle().resize_exact((430,430));
    canvas.draw_image(&img, (980,420), None);
    
    encode_png(surface.image_snapshot())
}

register_meme!(
    "yuzu_soft_murasame_finger",
    yuzu_soft_murasame_finger,
    min_images = 1,
    max_images = 1,
    keywords = &["柚子厨","丛雨指"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
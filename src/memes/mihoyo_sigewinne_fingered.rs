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

fn mihoyo_sigewinne_fingered(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("希格雯:这{}没救了\n希格雯:快拉去璃月往生堂\n希格雯:让胡堂主埋了吧", name);
    let frame = load_image("mihoyo_sigewinne_fingered/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制文字
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 345, 351, 435),
        &text,
        10.0,
        70.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    // 然后在同一个 canvas 上绘制图片
    let img = images[0].image.circle().resize_exact((144, 144));
    canvas.draw_image(&img, (12, 47), None);
    
    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_sigewinne_fingered",
    mihoyo_sigewinne_fingered,
    min_images = 1,
    max_images = 1,
    keywords = &["没救了","希格雯指"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
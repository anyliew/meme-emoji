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

fn doro_thumbs_up(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{},你真是一个大聪明\n桃乐丝要为你点个赞👍🏻", name);
    let frame = load_image("doro_thumbs_up/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制文字
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1, 797, 1072, 980),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    // 然后在同一个 canvas 上绘制图片
    let img = images[0].image.circle().resize_exact((230, 230));
    canvas.draw_image(&img, (730, 150), None);
    
    encode_png(surface.image_snapshot())
}

register_meme!(
    "doro_thumbs_up",
    doro_thumbs_up,
    min_images = 1,
    max_images = 1,
    keywords = &["doro点赞","Doro点赞","DORO点赞","桃乐丝点赞"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
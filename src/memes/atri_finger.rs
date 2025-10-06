use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn atri_finger(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name},你和夏先生一样笨得可爱");
    let frame = load_image("atri_finger/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 588, 1171, 720),
        &text,
        20.0,
        100.0,
        text_params!(
            paint = new_paint(Color::from_rgb(0, 0, 0)),
            text_align = TextAlign::Left,
            font_families = &["FZShaoEr-M11S"],
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((300, 300));
        canvas.draw_image(&img, (600, 140), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "atri_finger",
    atri_finger,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::atri(),
    keywords = &["亚托莉指"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

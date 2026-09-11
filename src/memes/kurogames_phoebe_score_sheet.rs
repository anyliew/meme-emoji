use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_phoebe_score_sheet(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = if !texts.is_empty() { &texts[0] } else { "他" };
    let text = format!("{},你这个月评分为0,纯纯的饭桶！", name);
    let frame = load_image("kurogames_phoebe_score_sheet/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(50, 50, 740, 280),
        &text,
        35.0,
        100.0,
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::BLACK),
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);

        canvas.draw_image(&frame, (0, 0), None);
        let image = images[0].circle().resize_exact((140, 140));
        canvas.draw_image(&image, (155, 855), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_phoebe_score_sheet",
    kurogames_phoebe_score_sheet,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["菲比评分表", "评分表"],
    date_created = local_date(2025, 5, 24),
    date_modified = local_date(2025, 5, 24),
);

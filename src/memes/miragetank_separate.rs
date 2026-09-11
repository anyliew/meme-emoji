use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn miragetank_separate(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let mut bright_factor = 2.0f32;
    if let Some(text) = texts.first() {
        let text = text.trim();
        if !text.is_empty() {
            if let Ok(value) = text.parse::<f32>() {
                bright_factor = value.clamp(0.1, 6.0);
            }
        }
    }

    let func = |images: Vec<Image>| {
        let tank = images[0].clone();
        let mut white_surface = new_surface(tank.dimensions());
        let white_canvas = white_surface.canvas();
        white_canvas.clear(Color::WHITE);
        white_canvas.draw_image(&tank, (0, 0), None);

        let mut black_surface = new_surface(tank.dimensions());
        let black_canvas = black_surface.canvas();
        black_canvas.clear(Color::BLACK);
        black_canvas.draw_image(&tank, (0, 0), None);

        let white_img = white_surface.image_snapshot();
        let black_img = black_surface.image_snapshot().brightness(bright_factor);

        let mut surface = new_surface((tank.width() * 2, tank.height()));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&white_img, (0, 0), None);
        canvas.draw_image(&black_img, (tank.width(), 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "miragetank_separate",
    miragetank_separate,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["幻影分离", "坦克分离"],
    date_created = local_date(2026, 3, 5),
    date_modified = local_date(2026, 3, 5),
);

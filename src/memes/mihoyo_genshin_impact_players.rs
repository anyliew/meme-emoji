use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mihoyo_genshin_impact_players(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_genshin_impact_players/0.png")?;
    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        canvas.draw_image(&frame, (0, 0), None);

        let image = images[0].circle().resize_fit((220, 220), Fit::Cover);
        canvas.draw_image(&image, (385, 120), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_genshin_impact_players",
    mihoyo_genshin_impact_players,
    tags = MemeTags::genshin(),
    min_images = 1,
    max_images = 1,
    keywords = &["原批", "原神玩家"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

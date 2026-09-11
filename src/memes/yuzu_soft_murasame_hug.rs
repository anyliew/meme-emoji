use rand::RngExt;
use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::number_option, register_meme};

number_option!(Number, 1, 3);

fn yuzu_soft_murasame_hug(images: Vec<InputImage>, _: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let num = options.number.unwrap_or(rand::rng().random_range(1..=3));
    let index = (num as usize) - 1;
    let files = ["0.png", "1.png", "2.png"];
    let positions = [(20, 146), (23, 185), (24, 188)];
    let sizes = [(118, 118), (156, 156), (156, 156)];
    let frame = load_image(format!("yuzu_soft_murasame_hug/{}", files[index]))?;
    let pos = positions[index];
    let size = sizes[index];
    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        let image = images[0].circle().resize_exact(size);
        canvas.draw_image(&image, pos, None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_png_or_gif(images, func)
}

register_meme!(
    "yuzu_soft_murasame_hug",
    yuzu_soft_murasame_hug,
    min_images = 1,
    max_images = 1,
    keywords = &["丛雨抱"],
    date_created = local_date(2026, 2, 19),
    date_modified = local_date(2026, 4, 1),
);

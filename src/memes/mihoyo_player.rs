use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mihoyo_player(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_player/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(
        &images[0].image.circle().resize_exact((312, 312)),
        (1073, 628),
        None,
    );
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_player",
    mihoyo_player,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["二游玩家"],
    tags = MemeTags::mihoyo(),
    date_created = local_date(2026, 9, 9),
    date_modified = local_date(2026, 9, 9),
);

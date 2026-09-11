use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_abby_weeping(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (2, 77), (2, 75), (2, 73), (2, 72), (2, 70),
        (2, 70), (2, 70), (2, 70), (2, 70), (2, 70),
        (2, 70), (2, 70), (2, 70), (2, 70), (2, 70),
        (2, 70),
    ];
    let image = images[0].image.circle().resize_exact((80, 80));
    let frames = (0..locs.len())
        .map(|i| load_image(format!("kurogames_abby_weeping/{i}.png")))
        .collect::<Result<Vec<_>, _>>()?;
    let frame_size = frames.iter().fold((0, 0), |(w, h), frame| {
        (w.max(frame.width()), h.max(frame.height()))
    });
    let mut encoder = GifEncoder::new();
    for (frame, (x, y)) in frames.iter().zip(locs.iter()) {
        let mut surface = new_surface(frame_size);
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        canvas.draw_image(&image, (*x, *y), None);
        canvas.draw_image(frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.06)?;
    }
    encoder.finish()
}

register_meme!(
    "kurogames_abby_weeping",
    kurogames_abby_weeping,
    min_images = 1,
    max_images = 1,
    keywords = &["抱头痛哭"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2025, 7, 15),
    date_modified = local_date(2025, 7, 15),
);

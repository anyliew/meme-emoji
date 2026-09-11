use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn estrous(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (105, 229), (104, 230), (103, 232), (101, 232), (98, 233),
        (96, 233), (93, 233), (91, 235), (89, 235), (86, 235),
        (85, 235), (84, 234), (83, 235), (81, 232), (82, 233),
        (82, 231), (85, 231), (87, 230), (89, 229), (92, 228),
        (94, 227), (97, 227), (100, 226), (101, 226), (104, 225),
        (106, 225), (106, 225), (109, 226), (109, 227), (108, 228),
        (108, 229), (106, 230), (105, 227),
    ];
    let image = images[0].image.resize_exact((98, 66));
    let frames = (0..locs.len())
        .map(|i| load_image(format!("estrous/{i}.png")))
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
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    encoder.finish()
}

register_meme!(
    "estrous",
    estrous,
    min_images = 1,
    max_images = 1,
    keywords = &["发情"],
    date_created = local_date(2025, 8, 11),
    date_modified = local_date(2025, 8, 11),
);

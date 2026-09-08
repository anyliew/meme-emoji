use skia_safe::Color;
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};
use crate::{options::NoOptions, register_meme};

fn shuai(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (83, 69, 43, 135),
        (53, 45, 49, 103),
        (53, 44, 138, 100),
        (61, 62, 149, 125),
    ];

    let mut encoder = GifEncoder::new();

    for i in 0..4 {
        let (w, h, x, y) = locs[i];
        let frame = load_image(format!("shuai/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        let img = images[0].image.square().resize_exact((w, h)).circle();
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);

        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }

    encoder.finish()
}

register_meme! {
    "shuai",
    shuai,
    min_images = 1,
    max_images = 1,
    keywords = &["甩"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 5, 27),
}
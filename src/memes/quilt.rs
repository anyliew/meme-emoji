use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn quilt(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (68, 68), (68, 68), (68, 72), (69, 74), (69, 74),
        (69, 73), (69, 72), (69, 74), (68, 72), (68, 68),
        (68, 68), (68, 68), (68, 72), (68, 72), (69, 73),
        (69, 73), (69, 73), (69, 75), (68, 72), (68, 68),
        (68, 68),
    ];
    
    let user_head = images[0].image.resize_exact((142, 146));

    let mut encoder = GifEncoder::new();
    
    for i in 0..21 {
        let frame = load_image(format!("quilt/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "quilt",
    quilt,
    min_images = 1,
    max_images = 1,
    keywords = &["被窝"],
    date_created = local_date(2025, 9, 12),
    date_modified = local_date(2025, 9, 12),
}
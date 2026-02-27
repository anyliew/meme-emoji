use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn frieren_snuggle(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (57, 132), (57, 132), (57, 137), (57, 137), (56, 138),
        (56, 138), (56, 138), (55, 140), (55, 140), (55, 140),
        (57, 137), (57, 137), (58, 135), (57, 132), (57, 132),
        (57, 132), (57, 132),
    ];
    
    let user_head = images[0].image.resize_exact((78, 78));

    let mut encoder = GifEncoder::new();
    
    for i in 0..17 {
        let frame = load_image(format!("frieren_snuggle/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "frieren_snuggle",
    frieren_snuggle,
    min_images = 1,
    max_images = 1,
    keywords = &["芙莉莲贴贴"],
    date_created = local_date(2026, 1, 20),
    date_modified = local_date(2026, 1, 20),
}
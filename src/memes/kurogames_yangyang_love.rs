use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_yangyang_love(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (141, 197), (142, 192), (141, 188), (142, 192), (142, 197),
        (142, 192), (141, 188), (142, 192)
    ];
    
    let user_head = images[0].image.resize_exact((130, 98));

    let mut encoder = GifEncoder::new();
    
    for i in 0..8 {
        let frame = load_image(format!("kurogames_yangyang_love/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "kurogames_yangyang_love",
    kurogames_yangyang_love,
    min_images = 1,
    max_images = 1,
    keywords = &["秧秧比心"],
    date_created = local_date(2025, 12, 1),
    date_modified = local_date(2025, 12, 1),
}
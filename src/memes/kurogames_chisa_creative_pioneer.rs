use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_chisa_creative_pioneer(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let position = (47, 47);
    
    let user_head = images[0].image
        .resize_exact((164, 164))
        .circle();

    let mut encoder = GifEncoder::new();
    
    for i in 0..90 {
        let frame = load_image(format!("kurogames_chisa_creative_pioneer/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = position;
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.07)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "kurogames_chisa_creative_pioneer",
    kurogames_chisa_creative_pioneer,
    min_images = 1,
    max_images = 1,
    keywords = &["创作先锋"],
    date_created = local_date(2026, 2, 16),
    date_modified = local_date(2026, 2, 16),
}
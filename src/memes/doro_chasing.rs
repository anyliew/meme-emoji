use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_chasing(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (268, 147), (267, 148), (268, 146), (267, 148)
    ];
    
    let user_head = images[0].image.resize_exact((96, 96));

    let mut encoder = GifEncoder::new();
    
    for i in 0..4 {
        let frame = load_image(format!("doro_chasing/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.07)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "doro_chasing",
    doro_chasing,
    min_images = 1,
    max_images = 1,
    keywords = &["doro追", "Doro追", "桃乐丝追"],
    date_created = local_date(2025, 12, 2),
    date_modified = local_date(2025, 12, 2),
}
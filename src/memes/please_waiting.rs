use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn please_waiting(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let position = (81, 76);
    
    let user_head = images[0].image.resize_exact((159, 159));

    let mut encoder = GifEncoder::new();
    
    for i in 0..100 {
        let frame = load_image(format!("please_waiting/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = position;
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "please_waiting",
    please_waiting,
    min_images = 1,
    max_images = 1,
    keywords = &["请稍候"],
    date_created = local_date(2026, 1, 9),
    date_modified = local_date(2026, 1, 9),
}
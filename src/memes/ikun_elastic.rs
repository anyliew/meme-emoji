use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ikun_elastic(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (89, 82), (90, 83), (87, 83), (87, 83), (87, 83),
        (87, 83), (86, 84), (87, 84), (89, 84), (92, 87),
        (95, 88), (94, 86), (88, 83)
    ];
    
    let user_head = images[0].image.resize_exact((112, 112));

    let mut encoder = GifEncoder::new();
    
    for i in 0..13 {
        let frame = load_image(format!("ikun_elastic/{}.png", i + 1))?;
        
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
    "ikun_elastic",
    ikun_elastic,
    min_images = 1,
    max_images = 1,
    keywords = &["弹坤"],
    date_created = local_date(2025, 12, 11),
    date_modified = local_date(2025, 12, 11),
}
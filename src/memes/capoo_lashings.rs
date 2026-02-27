use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn capoo_lashings(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (100, 102), (100, 102), (100, 102), (100, 102), (100, 102),
        (100, 102), (100, 102), (100, 102), (100, 102), (100, 102),
        (100, 102), (100, 102),
    ];
    
    let user_head = images[0].image.resize_exact((330, 330)).circle();

    let mut encoder = GifEncoder::new();
    
    for i in 0..12 {
        let frame = load_image(format!("capoo_lashings/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.02)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "capoo_lashings",
    capoo_lashings,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波捶打", "捶打"],
    date_created = local_date(2025, 12, 5),
    date_modified = local_date(2025, 12, 5),
}
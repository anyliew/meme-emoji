use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn tencent_marco_polo_ride(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (30, 32), (30, 32), (33, 29), (35, 27), (35, 27),
        (35, 27), (36, 27), (37, 27), (39, 28), (36, 36),
        (28, 50), (31, 73), (38, 76), (46, 75), (61, 70),
        (76, 65), (65, 69), (43, 76), (38, 65), (20, 84),
        (24, 110), (31, 115), (55, 68), (64, 42), (66, 51),
        (59, 84), (49, 85), (60, 74), (83, 51), (103, 37),
        (99, 59), (102, 59), (108, 46), (108, 53), (102, 73),
        (97, 80), (96, 71), (108, 52), (113, 60), (118, 59),
        (118, 59), (118, 59), (118, 59), (119, 57), (119, 57),
        (117, 59), (118, 58), (119, 57), (119, 57), (119, 57),
        (119, 58), (120, 57), (120, 57), (120, 57), (120, 57),
        (120, 57), (120, 57), (119, 58), (119, 58), (119, 58),
        (119, 58), (118, 60), (118, 59), (120, 57), (120, 57),
        (118, 59), (119, 59), (119, 59), (119, 59), (117, 61),
        (117, 61), (118, 58), (115, 58)
    ];
    
    let user_head = images[0].image.resize_exact((52, 52));

    let mut encoder = GifEncoder::new();
    
    for i in 0..73 {
        let frame = load_image(format!("tencent_marco_polo_ride/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.12)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "tencent_marco_polo_ride",
    tencent_marco_polo_ride,
    min_images = 1,
    max_images = 1,
    keywords = &["马克骑", "马可波罗骑"],
    date_created = local_date(2026, 1, 9),
    date_modified = local_date(2026, 1, 9),
}
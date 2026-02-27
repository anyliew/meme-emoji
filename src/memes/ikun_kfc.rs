use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn ikun_kfc(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (51, 51), (53, 51), (59, 44), (60, 0), (64, 0),
        (69, 36), (71, 33), (69, 36), (64, 0), (60, 0),
        (59, 44), (53, 51)
    ];
    
    let sizes = [
        (184, 121), (175, 121), (170, 119), (161, 161), (169, 159),
        (166, 122), (163, 118), (166, 122), (169, 159), (161, 161),
        (170, 119), (175, 121)
    ];

    let user_img = &images[0].image;

    let mut encoder = GifEncoder::new();
    
    for i in 0..12 {
        let frame = load_image(format!("ikun_kfc/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (width, height) = sizes[i];
        let (x, y) = positions[i];
        
        let user_head = user_img.resize_exact((width, height));
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "ikun_kfc",
    ikun_kfc,
    min_images = 1,
    max_images = 1,
    keywords = &["肯德坤"],
    date_created = local_date(2025, 12, 11),
    date_modified = local_date(2025, 12, 11),
}
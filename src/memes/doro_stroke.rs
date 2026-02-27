use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_stroke(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (148, 306), (148, 306), (148, 306), (148, 306), (148, 306),
        (148, 306), (148, 306), (148, 306), (148, 306), (148, 306),
        (148, 306), (148, 306), (148, 306), (148, 306), (148, 306),
        (148, 306),
    ];
    
    let user_head = images[0].image.resize_exact((81, 81));

    let mut encoder = GifEncoder::new();
    
    for i in 0..16 {
        let frame = load_image(format!("doro_stroke/{}.png", i + 1))?;
        
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
    "doro_stroke",
    doro_stroke,
    min_images = 1,
    max_images = 1,
    keywords = &["抚摸", "doro抚摸", "Doro抚摸", "桃乐丝抚摸"],
    date_created = local_date(2026, 1, 15),
    date_modified = local_date(2026, 1, 15),
}
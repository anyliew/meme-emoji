use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (0, 600), (0, 645), (0, 680), (0, 720),
    ];
    
    let user_head = images[0].image.resize_exact((640, 640));

    let mut encoder = GifEncoder::new();
    
    for i in 0..4 {
        let frame = load_image(format!("doro_do/{}.png", i + 1))?;
        
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
    "doro_do",
    doro_do,
    min_images = 1,
    max_images = 1,
    keywords = &["doro撅", "Doro撅", "桃乐丝撅"],
    date_created = local_date(2026, 1, 15),
    date_modified = local_date(2026, 1, 15),
}
use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doro_banging(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (149, 109), (149, 109), (149, 109), (139, 133), (134, 140),
        (143, 127), (149, 109), (149, 109), (149, 109), (149, 109),
        (149, 109)
    ];

    let sizes = [
        (82, 82), (82, 82), (82, 82), (102, 52), (112, 45),
        (89, 60), (82, 82), (82, 82), (82, 82), (82, 82),
        (82, 82)
    ];

    let user_img = &images[0].image;

    let mut encoder = GifEncoder::new();
    
    for i in 0..11 {
        let frame = load_image(format!("doro_banging/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (width, height) = sizes[i];
        let (x, y) = positions[i];
        
        let user_head = user_img.resize_exact((width, height));
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "doro_banging",
    doro_banging,
    min_images = 1,
    max_images = 1,
    keywords = &["doro敲", "Doro敲", "桃乐丝敲"],
    date_created = local_date(2026, 1, 20),
    date_modified = local_date(2026, 1, 20),
}
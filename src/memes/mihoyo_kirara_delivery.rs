use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_kirara_delivery(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (226, 74), (226, 74), (226, 74), (226, 74)
    ];

    let sizes = [
        (141, 120), (141, 120), (141, 120), (141, 120)
    ];

    let user_img = &images[0].image;

    let mut encoder = GifEncoder::new();
    
    for i in 0..4 {
        let frame = load_image(format!("mihoyo_kirara_delivery/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (width, height) = sizes[i];
        let (x, y) = positions[i];
        
        let user_head = user_img.resize_exact((width, height));
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "mihoyo_kirara_delivery",
    mihoyo_kirara_delivery,
    min_images = 1,
    max_images = 1,
    keywords = &["绮良良派送", "派送"],
    date_created = local_date(2025, 12, 5),
    date_modified = local_date(2025, 12, 5),
}
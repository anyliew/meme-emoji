use skia_safe::Color;
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};
use crate::{options::NoOptions, register_meme};

fn capoo_gothammered(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (80, 80, 82, 147), (80, 80, 77, 116), (80, 80, 79, 128), (80, 80, 76, 135), (80, 80, 80, 146),
        (80, 80, 77, 127), (80, 80, 80, 127), (80, 80, 78, 137), (80, 80, 81, 146), (80, 80, 78, 130),
        (80, 80, 81, 129), (80, 80, 81, 139), (80, 80, 81, 147), (80, 80, 80, 131), (80, 80, 81, 130),
        (80, 80, 79, 137), (80, 80, 81, 146), (80, 80, 78, 131), (80, 80, 79, 130), (80, 80, 80, 138)
    ];
    
    let user_head = images[0].image.square().resize_exact((80, 80));
    let mut encoder = GifEncoder::new();
    
    for i in 0..20 {
        let (_w, _h, x, y) = locs[i];
        let frame_num = (i % 20) + 1;
        let frame = load_image(format!("capoo_gothammered/{frame_num}.png"))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "capoo_gothammered",
    capoo_gothammered,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波被锤"],
    date_created = local_date(2025, 12, 5),
    date_modified = local_date(2025, 12, 5),
}
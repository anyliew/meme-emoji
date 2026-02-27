use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_camellya_ride(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (101, 200), (100, 203), (98, 209), (93, 208), (84, 209),
        (77, 213), (77, 224), (87, 221), (104, 223), (109, 223),
        (106, 230), (92, 227), (75, 236), (70, 233), (71, 239),
        (81, 224), (92, 214), (96, 210), (92, 216), (78, 210),
        (67, 223), (71, 220), (74, 228), (90, 216), (111, 224),
        (113, 216), (101, 222), (80, 223), (73, 233), (79, 235),
        (94, 224), (116, 232), (117, 226), (108, 231), (99, 227),
        (87, 226), (88, 227), (94, 230), (107, 230), (108, 235),
        (107, 233), (97, 225), (80, 232), (73, 243), (79, 239),
        (95, 236), (121, 237), (130, 233), (124, 234), (118, 235)
    ];
    
    let user_head = images[0].image.resize_exact((48, 48));

    let mut encoder = GifEncoder::new();
    
    for i in 0..11 {
        let frame = load_image(format!("kurogames_camellya_ride/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "kurogames_camellya_ride",
    kurogames_camellya_ride,
    min_images = 1,
    max_images = 1,
    keywords = &["椿骑", "大傻椿骑"],
    date_created = local_date(2026, 2, 16),
    date_modified = local_date(2026, 2, 16),
}
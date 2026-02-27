use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_mornye_raise(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (92, 281), (92, 281), (91, 280), (91, 279), (91, 279),
        (90, 277), (89, 276), (88, 274), (87, 272), (86, 270),
        (85, 269), (84, 267), (84, 266), (83, 264), (82, 263),
        (82, 263), (81, 263), (81, 263), (81, 263), (81, 263),
        (81, 263), (81, 263), (81, 263), (81, 263), (81, 263),
        (81, 263), (81, 263), (81, 263), (81, 263)
    ];

    let user_img = &images[0].image;

    let mut encoder = GifEncoder::new();
    
    for i in 0..29 {
        let frame = load_image(format!("kurogames_mornye_raise/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        // 调整头像尺寸并可能旋转
        let mut user_head = user_img.resize_exact((176, 176));
        
        // 对于前13帧（索引0-12），将头像旋转13°
        if i < 13 {
            user_head = user_head.rotate(-13.0);
        }
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "kurogames_mornye_raise",
    kurogames_mornye_raise,
    min_images = 1,
    max_images = 1,
    keywords = &["莫宁举"],
    date_created = local_date(2026, 1, 20),
    date_modified = local_date(2026, 1, 20),
}
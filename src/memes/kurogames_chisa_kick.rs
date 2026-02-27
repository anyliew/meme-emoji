use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_chisa_kick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (321, 82), (321, 82), (320, 77), (321, 80), (321, 85),
        (322, 90), (321, 85), (317, 78), (313, 78), (309, 81),
        (307, 85), (306, 83), (303, 79), (301, 78), (298, 79),
        (294, 81), (294, 84), (294, 91), (303, 97), (329, 99),
        (368, 102), (397, 105), (397, 105), (397, 105)
    ];
    
    let user_head = images[0].image.resize_exact((84, 84));

    let mut encoder = GifEncoder::new();
    
    for i in 0..33 {
        let frame = load_image(format!("kurogames_chisa_kick/{}.png", i + 1))?;
        
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        canvas.clear(Color::TRANSPARENT);
        
        // 只有前24帧需要贴头像
        if i < positions.len() {
            let (x, y) = positions[i];
            canvas.draw_image(&user_head, (x, y), None);
        }
        
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "kurogames_chisa_kick",
    kurogames_chisa_kick,
    min_images = 1,
    max_images = 1,
    keywords = &["千咲踢"],
    date_created = local_date(2025, 12, 1),
    date_modified = local_date(2025, 12, 30),
}
use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_phrolova_boxer(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (164, 162), (143, 151), (165, 163), (141, 152),
    ];
    
    // 修复：使用 resize_exact 替代 resize
    // 注意：Python 版本使用了 keep_ratio=True，但在 Rust 中可能需要不同的处理方式
    // 这里直接使用 resize_exact 来精确控制尺寸
    let user_head = images[0].image.resize_exact((100, 100));

    let mut encoder = GifEncoder::new();
    
    for i in 0..4 {
        let frame = load_image(format!("kurogames_phrolova_boxer/{}.png", i + 1))?;
        
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
    "kurogames_phrolova_boxer",
    kurogames_phrolova_boxer,
    min_images = 1,
    max_images = 1,
    keywords = &["弗洛洛拳击"],
    date_created = local_date(2026, 1, 12),
    date_modified = local_date(2026, 1, 12),
}
use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_aino_ride(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_aino_ride/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        
        canvas.draw_image(&frame, (0, 0), None);
        
        // 位置1
        let img1 = images[0].resize_exact((224, 199));
        canvas.draw_image(&img1, (593, 721), None);
        
        // 位置2
        let img2 = images[0].resize_exact((132, 144));
        canvas.draw_image(&img2, (262, 832), None);
        
        // 位置3
        let img3 = images[0].resize_exact((115, 115));
        canvas.draw_image(&img3, (390, 754), None);
        
        // 位置4
        let img4 = images[0].resize_exact((111, 125));
        canvas.draw_image(&img4, (133, 692), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_aino_ride",
    mihoyo_aino_ride,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["爱诺骑"],
    date_created = local_date(2025, 12, 2),
    date_modified = local_date(2025, 12, 2),
);
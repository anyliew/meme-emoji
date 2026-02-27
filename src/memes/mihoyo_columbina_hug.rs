use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_columbina_hug(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_columbina_hug/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        
        let img = images[0]
            .resize_exact((467, 432));
            
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, (510, 470), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo_columbina_hug",
    mihoyo_columbina_hug,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["少女抱", "哥伦比娅抱", "月神抱"],
    date_created = local_date(2025, 12, 2),
    date_modified = local_date(2025, 12, 2),
);
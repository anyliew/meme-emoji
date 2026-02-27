use skia_safe::{Color, IRect, Image, textlayout::TextAlign};
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};
use crate::{options::NoOptions, register_meme};

fn fbi_photo(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let ta = "她";
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else {
        ta.to_string()
    };
    
    let text = format!("Name : {}", name);
    
    let frame = load_image("fbi_photo/0.png")?;

    let func = |images: Vec<Image>| -> Result<Image, Error> {
        let img = images[0].resize_fit((220, 220), Fit::Cover);
        
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        canvas.draw_image(&frame, (0, 0), None);
        
        canvas.draw_image(&img, (985, 400), None);
        
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(800, 690, 1200, 790),
            &text,
            20.0,
            70.0,
            text_params!(
                font_families = &[],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fbi_photo",
    fbi_photo,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["fbi", "FBI"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
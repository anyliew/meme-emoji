use skia_safe::{AlphaType, image::CachingHint, Color, ColorType, Data, Image, ImageInfo, images};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn read_rgba(image: &Image) -> Result<Vec<u8>, Error> {
    let info = ImageInfo::new(
        image.dimensions(),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    let row_bytes = info.min_row_bytes();
    let mut data = vec![0u8; info.compute_min_byte_size()];
    if !image.read_pixels(&info, &mut data, row_bytes, (0, 0), CachingHint::Allow) {
        return Err(Error::ImageDecodeError("read pixels failed".to_string()));
    }
    Ok(data)
}

fn image_from_rgba(image: &Image, data: Vec<u8>) -> Result<Image, Error> {
    let info = ImageInfo::new(
        image.dimensions(),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    let row_bytes = info.min_row_bytes();
    images::raster_from_data(&info, Data::new_copy(&data), row_bytes)
        .ok_or_else(|| Error::ImageEncodeError("create image failed".to_string()))
}

fn luma(data: &[u8], index: usize) -> u8 {
    (0.299 * data[index] as f32 + 0.587 * data[index + 1] as f32 + 0.114 * data[index + 2] as f32)
        as u8
}

fn miragetank_gray(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |images: Vec<Image>| {
        let cover = images[0].clone();
        let base = images[1].clone();
        let width = cover.width() as usize;
        let height = cover.height() as usize;

        let cover_data = read_rgba(&cover)?;

        let base_thumb = base.resize_bound((width as i32, height as i32), Fit::Cover);
        let mut base_surface = new_surface((width as i32, height as i32));
        let base_canvas = base_surface.canvas();
        base_canvas.clear(Color::TRANSPARENT);
        base_canvas.draw_image(
            &base_thumb,
            (
                (width as i32 - base_thumb.width()) / 2,
                (height as i32 - base_thumb.height()) / 2,
            ),
            None,
        );
        let base_data = read_rgba(&base_surface.image_snapshot())?;

        let mut out = vec![0u8; width * height * 4];
        for i in 0..(width * height) {
            let index = i * 4;
            let w = luma(&cover_data, index) as f32 * 0.5 + 128.0;
            let b = luma(&base_data, index) as f32 * 0.5;
            let a = 1.0 - w / 255.0 + b / 255.0;
            let r = if a.abs() > 1e-6 { b / a } else { 255.0 };
            let rgb = r.clamp(0.0, 255.0) as u8;
            let alpha = (a * 255.0).clamp(0.0, 255.0) as u8;
            out[index] = rgb;
            out[index + 1] = rgb;
            out[index + 2] = rgb;
            out[index + 3] = alpha;
        }

        image_from_rgba(&cover, out)
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "miragetank_gray",
    miragetank_gray,
    min_images = 2,
    max_images = 2,
    min_texts = 0,
    max_texts = 0,
    keywords = &["灰色幻影坦克", "灰幻"],
    date_created = local_date(2026, 3, 5),
    date_modified = local_date(2026, 3, 5),
);

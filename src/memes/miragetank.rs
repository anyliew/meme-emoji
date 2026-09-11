use skia_safe::{AlphaType, image::CachingHint, ColorType, Data, Image, ImageInfo, images};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::local_date,
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

fn miragetank(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |images: Vec<Image>| {
        let top = images[0].clone();
        let bottom = if images[1].dimensions() == top.dimensions() {
            images[1].clone()
        } else {
            images[1].resize_exact(top.dimensions())
        };

        let top_data = read_rgba(&top)?;
        let bottom_data = read_rgba(&bottom)?;
        let width = top.width() as usize;
        let height = top.height() as usize;
        let mut out = vec![0u8; width * height * 4];

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) * 4;
                let top_l = luma(&top_data, index);
                let bottom_l = luma(&bottom_data, index);
                let (l, a) = if (x + y) % 2 == 0 {
                    (255u8, 255u8 - top_l)
                } else {
                    (0u8, 255u8 - bottom_l)
                };
                out[index] = l;
                out[index + 1] = l;
                out[index + 2] = l;
                out[index + 3] = a;
            }
        }

        image_from_rgba(&top, out)
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "miragetank",
    miragetank,
    min_images = 2,
    max_images = 2,
    min_texts = 0,
    max_texts = 0,
    keywords = &["幻影坦克"],
    date_created = local_date(2026, 3, 5),
    date_modified = local_date(2026, 3, 5),
);

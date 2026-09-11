use skia_safe::{AlphaType, image::CachingHint, Color, ColorType, Data, Image, ImageInfo, images};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const WLIGHT: f32 = 1.0;
const BLIGHT: f32 = 0.18;
const WCOLOR: f32 = 0.5;
const BCOLOR: f32 = 0.7;

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

fn miragetank_color(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
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
            let mut w = [
                cover_data[index] as f32 / 255.0 * WLIGHT,
                cover_data[index + 1] as f32 / 255.0 * WLIGHT,
                cover_data[index + 2] as f32 / 255.0 * WLIGHT,
            ];
            let mut b = [
                base_data[index] as f32 / 255.0 * BLIGHT,
                base_data[index + 1] as f32 / 255.0 * BLIGHT,
                base_data[index + 2] as f32 / 255.0 * BLIGHT,
            ];
            let wgray = w[0] * 0.334 + w[1] * 0.333 + w[2] * 0.333;
            let bgray = b[0] * 0.334 + b[1] * 0.333 + b[2] * 0.333;
            for c in 0..3 {
                w[c] = w[c] * WCOLOR + wgray * (1.0 - WCOLOR);
                b[c] = b[c] * BCOLOR + bgray * (1.0 - BCOLOR);
            }
            let d = [
                1.0 - w[0] + b[0],
                1.0 - w[1] + b[1],
                1.0 - w[2] + b[2],
            ];
            let d_luma = d[0] * 0.222 + d[1] * 0.707 + d[2] * 0.071;
            for c in 0..3 {
                let p = if d_luma.abs() > 1e-6 {
                    b[c] / d_luma * 255.0
                } else {
                    255.0
                };
                out[index + c] = p.clamp(0.0, 255.0) as u8;
            }
            out[index + 3] = (d_luma * 255.0).clamp(0.0, 255.0) as u8;
        }

        image_from_rgba(&cover, out)
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "miragetank_color",
    miragetank_color,
    min_images = 2,
    max_images = 2,
    min_texts = 0,
    max_texts = 0,
    keywords = &["彩色幻影坦克", "彩幻"],
    date_created = local_date(2026, 3, 5),
    date_modified = local_date(2026, 3, 5),
);

use rand::RngExt;
use skia_safe::{AlphaType, image::CachingHint, ColorType, Data, Image, ImageInfo, images};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::number_option, register_meme};

number_option!(Number, 1, 7);

const SHADE_LIGHT: f32 = 108.0;
const LIGHT_CUT: f32 = 128.0;
const DARK_CUT: f32 = 118.0;
const SHADE_LIMIT: u8 = 120;
const TEXTURE_INTENSITY: f32 = 0.4;

const STOPS: [(f32, (f32, f32, f32)); 6] = [
    (0.0, (0.984, 0.729, 0.188)),
    (0.4, (0.988, 0.447, 0.208)),
    (0.6, (0.988, 0.208, 0.306)),
    (0.7, (0.812, 0.212, 0.875)),
    (0.8, (0.216, 0.710, 0.851)),
    (1.0, (0.243, 0.714, 0.855)),
];

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

fn make_kernel(mode: i32) -> (usize, Vec<f32>) {
    match mode {
        6 => (
            3,
            vec![1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
        ),
        _ => {
            let size = match mode {
                1 => 5,
                2 => 7,
                3 => 9,
                4 => 11,
                _ => 13,
            };
            let value = 1.0 / (size * size) as f32;
            (size, vec![value; size * size])
        }
    }
}

fn convolve(src: &[u8], width: usize, height: usize, kernel_size: usize, kernel: &[f32]) -> Vec<u8> {
    let radius = kernel_size as i32 / 2;
    let mut out = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut acc = 0.0f32;
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let sx = (x as i32 + kx as i32 - radius).clamp(0, width as i32 - 1) as usize;
                    let sy = (y as i32 + ky as i32 - radius).clamp(0, height as i32 - 1) as usize;
                    acc += src[sy * width + sx] as f32 * kernel[ky * kernel_size + kx];
                }
            }
            out[y * width + x] = acc.clamp(0.0, 255.0) as u8;
        }
    }
    out
}

fn box_blur(src: &[u8], width: usize, height: usize, radius: i32) -> Vec<u8> {
    let mut out = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0u32;
            let mut count = 0u32;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let sx = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;
                    let sy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    sum += src[sy * width + sx] as u32;
                    count += 1;
                }
            }
            out[y * width + x] = (sum / count) as u8;
        }
    }
    out
}

fn gradient(width: usize, height: usize) -> Vec<(u8, u8, u8)> {
    let w = width as f32;
    let h = height as f32;
    let denom = w * w + h * h;
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let t = ((x as f32 * w + y as f32 * h) / denom).clamp(0.0, 1.0);
            let mut i = 0;
            while i < STOPS.len() - 1 && t > STOPS[i + 1].0 {
                i += 1;
            }
            let (t0, c0) = STOPS[i];
            let (t1, c1) = STOPS[if i + 1 < STOPS.len() { i + 1 } else { i }];
            let f = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            out.push((
                ((c0.0 + (c1.0 - c0.0) * f) * 255.0) as u8,
                ((c0.1 + (c1.1 - c0.1) * f) * 255.0) as u8,
                ((c0.2 + (c1.2 - c0.2) * f) * 255.0) as u8,
            ));
        }
    }
    out
}

fn louvre(images: Vec<InputImage>, _: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let mode = options.number.unwrap_or(rand::rng().random_range(1..=7));
    let (kernel_size, kernel) = make_kernel(mode);

    let pencil_image = load_image("louvre/pencil-texture.jpg")?;

    let func = |images: Vec<Image>| {
        let img = images[0].clone();
        let width = img.width() as usize;
        let height = img.height() as usize;
        let data = read_rgba(&img)?;

        let pencil = pencil_image.resize_exact((width as i32, height as i32));
        let pencil_data = read_rgba(&pencil)?;
        let mut pencil_gray = vec![0u8; width * height];
        for i in 0..(width * height) {
            pencil_gray[i] = luma(&pencil_data, i * 4);
        }

        let mut plane = vec![0u8; width * height];
        for i in 0..(width * height) {
            let l = luma(&data, i * 4) as f32;
            let a = data[i * 4 + 3] as f32 / 255.0;
            plane[i] = (l * a + 255.0 * (1.0 - a)) as u8;
        }

        let mut shade = vec![0u8; width * height];
        for i in 0..(width * height) {
            shade[i] = if plane[i] > SHADE_LIMIT { 0 } else { 255 };
        }
        shade = box_blur(&shade, width, height, 3);
        for i in 0..(width * height) {
            let v = shade[i] as f32 * (255.0 - pencil_gray[i] as f32) / 255.0;
            shade[i] = (v * SHADE_LIGHT / 255.0) as u8;
        }

        let denoise_kernel = vec![1.0f32 / 9.0; 9];
        let plane = convolve(&plane, width, height, 3, &denoise_kernel);
        let blurred = convolve(&plane, width, height, kernel_size, &kernel);

        let mut mask = vec![0u8; width * height];
        for i in 0..(width * height) {
            let diff = (plane[i] as f32 - blurred[i] as f32 + 128.0).clamp(0.0, 255.0);
            let scale = (255.0 - LIGHT_CUT - DARK_CUT) / 255.0;
            let value = ((diff - DARK_CUT) * scale).clamp(0.0, 255.0);
            mask[i] = (255.0 - value).max(shade[i] as f32) as u8;
        }

        let grad = gradient(width, height);
        let mut out = vec![0u8; width * height * 4];
        for i in 0..(width * height) {
            let m = mask[i] as f32 / 255.0;
            let (gr, gg, gb) = grad[i];
            let mut r = gr as f32 * m + 255.0 * (1.0 - m);
            let mut g = gg as f32 * m + 255.0 * (1.0 - m);
            let mut b = gb as f32 * m + 255.0 * (1.0 - m);

            let tex_val = 255 - pencil_gray[i];
            if tex_val > 100 {
                let alpha = 100.0 * TEXTURE_INTENSITY / 255.0;
                r = ((r - 20.0).max(0.0)) * alpha + r * (1.0 - alpha);
                g = ((g - 20.0).max(0.0)) * alpha + g * (1.0 - alpha);
                b = ((b - 20.0).max(0.0)) * alpha + b * (1.0 - alpha);
            }

            let index = i * 4;
            out[index] = r.clamp(0.0, 255.0) as u8;
            out[index + 1] = g.clamp(0.0, 255.0) as u8;
            out[index + 2] = b.clamp(0.0, 255.0) as u8;
            out[index + 3] = 255;
        }

        image_from_rgba(&img, out)
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "louvre",
    louvre,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["卢浮宫"],
    date_created = local_date(2025, 5, 29),
    date_modified = local_date(2026, 5, 26),
);

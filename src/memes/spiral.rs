use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const CANVAS_SIZE: f32 = 600.0;
const CENTER: f32 = CANVAS_SIZE / 2.0;
const NUM_LAYERS: i32 = 7;
const PIGS_PER_LAYER: i32 = 12;
const TOTAL_FRAMES: u32 = 50;

fn spiral(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let base_img = images[0].clone();
        let mut surface = new_surface((CANVAS_SIZE as i32, CANVAS_SIZE as i32));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        let base_width = CANVAS_SIZE * 0.22;
        for layer in (0..NUM_LAYERS).rev() {
            let layer_f = layer as f32;
            let layer_scale_base = 0.70f32.powf(layer_f);
            let current_radius = 210.0 * layer_scale_base.powf(1.4);
            let size_factor = if (1..=3).contains(&layer) {
                (1.0 - layer_f * 0.14).powf(3.2)
            } else {
                (1.0 - layer_f * 0.14).powf(2.6)
            };
            let current_w = (base_width * size_factor).max(5.0);
            let current_h = current_w * base_img.height() as f32 / base_img.width() as f32;

            let pad_w = (base_img.width() as f32 * 1.3) as i32;
            let pad_h = (base_img.height() as f32 * 1.3) as i32;
            let mut pad_surface = new_surface((pad_w, pad_h));
            let pad_canvas = pad_surface.canvas();
            pad_canvas.draw_image(
                &base_img,
                ((pad_w - base_img.width()) / 2, (pad_h - base_img.height()) / 2),
                None,
            );
            let pig = pad_surface.image_snapshot().resize_exact((
                current_w as i32,
                current_h as i32,
            ));

            let direction: f32 = if layer % 2 == 0 { 1.0 } else { -1.0 };
            let frame_rotation = i as f32 * (360.0 / TOTAL_FRAMES as f32) * direction;
            for p in 0..PIGS_PER_LAYER {
                let angle_deg = p as f32 * (360.0 / PIGS_PER_LAYER as f32) + frame_rotation;
                let angle_rad = angle_deg.to_radians();
                let x = CENTER + current_radius * angle_rad.cos() - pig.width() as f32 / 2.0;
                let y = CENTER + current_radius * angle_rad.sin() - pig.height() as f32 / 2.0;
                let rotated_pig = pig.rotate_crop(angle_deg);
                canvas.draw_image(&rotated_pig, (x as i32, y as i32), None);
            }
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: TOTAL_FRAMES,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "spiral",
    spiral,
    min_images = 1,
    max_images = 1,
    keywords = &["螺旋转", "漩涡"],
    date_created = local_date(2026, 5, 28),
    date_modified = local_date(2026, 5, 28),
);

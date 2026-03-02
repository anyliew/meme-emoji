use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn cooking(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let params = [
        (80, 59), (80, 59), (80, 61), (80, 61), (80, 67),  //1-5
        (80, 67), (80, 59), (80, 59), (80, 61), (80, 62),  //6-10
        (80, 67), (80, 67), (80, 59), (80, 58), (80, 61),  //11-15
        (80, 62), (80, 67), (80, 67), (80, 58), (79, 59),  //16-20
        (80, 61), (80, 61), (80, 67), (80, 67), (80, 59),  //21-25
        (80, 59), (80, 61), (80, 61), (80, 67), (80, 67),  //26-30
        (80, 59), (80, 59), (80, 61), (80, 62), (80, 67),  //31-35
        (80, 67), (80, 58), (80, 59), (80, 63), (80, 64),  //36-40
        (78, 68), (79, 68), (80, 64), (80, 63), (80, 58),  //41-45
        (80, 59), (80, 64), (80, 63), (79, 67), (79, 67),  //46-50
        (80, 63), (80, 63),  //51-52
    ];

    let user_head = images[0].image.circle().resize_exact((95, 95));

    let func = |i: usize, _: Vec<Image>| {
        let frame = load_image(format!("cooking/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, params[i], None);
        canvas.draw_image(&frame, (0,0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 52,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "cooking",
    cooking,
    min_images = 1,
    max_images = 1,
    keywords = &["炒菜"],
    date_created = local_date(2025, 9, 29),
    date_modified = local_date(2025, 9, 29),
);

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn lochi_mari_play(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {

    let params = [
        (85, 178), (85, 178), (88, 177), (91, 178), (91, 178),  //1-5
        (86, 178), (86, 178), (86, 178), (86, 178), (86, 178),  //6-10
        (88, 178), (91, 179), (91, 179), (91, 179), (90, 178),  //11-15
        (87, 178),  // 16
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("lochi_mari_play/{i}.png"))?;
        let user_head = images[0].circle().resize_exact((64, 64));
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
            frame_num: 16,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "lochi_mari_play",
    lochi_mari_play,
    min_images = 1,
    max_images = 1,
    keywords = &["玛丽玩","伊落玛丽玩"],
    date_created = local_date(2025, 8, 10),
    date_modified = local_date(2025, 8, 10),
);

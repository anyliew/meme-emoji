use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_verina_play(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (72, 94), (69, 77), (77, 66), (73, 57), (68, 34),
        (69, 8), (68, -2), (67, -2), (66, -2), (69, -2),
        (71, -2), (73, -2), (73, -2), (71, -2), (66, -2),
        (64, 1), (65, 44), (69, 84),
    ];
    let sizes = [
        (186, 110), (191, 125), (184, 125), (194, 116), (203, 115),
        (203, 115), (209, 110), (211, 91), (212, 75), (206, 64),
        (201, 59), (197, 62), (195, 68), (194, 79), (196, 100),
        (198, 128), (197, 124), (195, 119),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_verina_play/{i}.png"))?;
        let head = images[0].square().resize_exact(sizes[i]);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 18,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_verina_play",
    kurogames_verina_play,
    min_images = 1,
    max_images = 1,
    keywords = &["维里奈顶", "小维顶"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 7, 21),
    date_modified = local_date(2026, 7, 21),
);

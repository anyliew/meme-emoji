use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_phrolova_bubble(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (294, 293), (305, 292), (311, 290), (316, 287), (320, 283),
        (323, 277), (325, 269), (325, 259), (324, 247), (323, 233),
        (319, 217), (317, 202), (315, 187), (314, 172), (310, 155),
        (310, 143), (311, 131), (313, 121), (314, 110), (315, 100),
        (318, 93), (322, 86), (323, 78), (327, 72), (328, 66),
        (331, 61), (334, 57), (334, 52), (335, 49), (336, 46),
        (334, 42), (334, 40), (334, 38), (334, 37), (331, 34),
        (330, 34), (341, 44), (283, 278), (282, 281), (281, 283),
        (282, 286), (281, 287), (294, 291), (294, 293),
    ];
    let sizes = [
        (135, 135), (135, 135), (135, 135), (135, 135), (135, 135),
        (135, 135), (135, 135), (136, 136), (136, 136), (136, 136),
        (140, 140), (140, 140), (140, 140), (140, 140), (147, 147),
        (147, 147), (147, 147), (148, 148), (150, 150), (153, 153),
        (153, 153), (153, 153), (157, 157), (157, 157), (161, 161),
        (161, 161), (161, 161), (165, 165), (165, 165), (165, 165),
        (169, 169), (169, 169), (169, 169), (169, 169), (172, 172),
        (172, 172), (148, 148), (135, 134), (141, 133), (144, 134),
        (143, 133), (143, 134), (130, 130), (134, 134),
    ];
    let extra_positions = [
        (295, 257), (289, 260), (287, 265), (287, 268), (283, 271), (281, 275),
    ];
    let extra_sizes = [
        (74, 138), (80, 136), (93, 137), (104, 135), (121, 137), (130, 136),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_phrolova_bubble/{i}.png"))?;
        let head1 = images[0].square().resize_exact(sizes[i]);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head1, positions[i], None);
        if (31..=36).contains(&i) {
            let j = i - 31;
            let head2 = images[0].square().resize_exact(extra_sizes[j]);
            canvas.draw_image(&head2, extra_positions[j], None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 44,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_phrolova_bubble",
    kurogames_phrolova_bubble,
    min_images = 1,
    max_images = 1,
    keywords = &["弗洛洛吹泡泡", "吹泡泡"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 6, 19),
    date_modified = local_date(2026, 6, 19),
);

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

fn doro_work_for_you(images: Vec<InputImage>, _texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("doro_work_for_you/0.png")?;
    let name = &images[0].name;
    let text = format!("{},桃乐丝正在努力为你打工赚钱", name);

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        let img = &images[0];
        
        // 第一次粘贴：大小170x170，位置(12, 7)，旋转37.5度
        let img1 = img.resize_bound((170, 170), Fit::Contain).rotate(-37.5);
        canvas.draw_image(&img1, (12, 7), None);
        
        // 第二次粘贴：大小90x90，位置(313, 130)，旋转-25度
        let img2 = img.resize_bound((90, 90), Fit::Contain).rotate(25.0);
        canvas.draw_image(&img2, (313, 130), None);
        
        // 第三次粘贴：大小35x35，位置(60, 450)，旋转35度
        let img3 = img.resize_bound((35, 35), Fit::Contain).rotate(-35.0);
        canvas.draw_image(&img3, (60, 450), None);
        
        // 然后绘制frame覆盖在用户图片之上
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字在最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 520, 1077, 575),
            &text,
            30.0,  // min_fontsize
            60.0,  // max_fontsize
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "doro_work_for_you",
    doro_work_for_you,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["为你打工"],
    date_created = local_date(2025, 9, 2),
    date_modified = local_date(2025, 9, 2),
);
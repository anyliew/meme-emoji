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

fn doro_surrounding_photos(images: Vec<InputImage>, _texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("doro_surrounding_photos/0.png")?;
    let name = &images[0].name;
    let text = format!("桃乐丝:我这边里有Doro动漫周边公仔\n桃乐丝:还有{}的写真~\n桃乐丝:你要吗？", name);

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        let img = &images[0];
        
        // 第一次粘贴 - 先绘制用户图片在最底层
        let img1 = img.resize_bound((100, 130), Fit::Contain);
        canvas.draw_image(&img1, (860, 1095), None);
        
        // 第二次粘贴
        let img2 = img.resize_bound((68, 68), Fit::Contain);
        canvas.draw_image(&img2, (864, 1227), None);
        
        // 第三次粘贴
        let img3 = img.resize_bound((100, 100), Fit::Contain).rotate(15.0);
        canvas.draw_image(&img3, (815, 1296), None);
        
        // 然后绘制frame覆盖在用户图片之上
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字在最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 1, 1080, 250),
            &text,
            30.0,  // min_fontsize
            60.0,  // max_fontsize
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "doro_surrounding_photos",
    doro_surrounding_photos,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["周边写真"],
    date_created = local_date(2025, 9, 13),
    date_modified = local_date(2025, 9, 13),
);
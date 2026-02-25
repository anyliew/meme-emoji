use skia_safe::{Color};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn look_leg(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
     let locs = [
        (73, 73, 30, 51),
        (73, 73, 30, 51),
        (73, 73, 30, 51),
        (73, 73, 30, 51),
        (73, 73, 30, 51),
        (73, 73, 30, 51),
        (73, 73, 30, 51),
        (73, 73, 30, 51)
        ];

    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();

    // 循环生成数量帧动画
    for i in 1..8 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];

        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("look_leg/{i}.png"))?;

        // 创建与帧图像相同尺寸的画布
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();

        // 清空画布为白色背景
        canvas.clear(Color::WHITE);

        // 调整输入图像到指定尺寸
        let image = image.resize_exact((w, h));

        // 在画布上绘制调整后的输入图像
        canvas.draw_image(&image, (x, y), None);

        // 在画布上绘制咖波贴图（覆盖在输入图像上方）
        canvas.draw_image(&frame, (0, 0), None);


        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }

    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

register_meme!(
    "look_leg",
    look_leg,
    min_images = 1,
    max_images = 1,
    keywords = &["看看腿"],
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
);

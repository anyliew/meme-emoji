// 导入所需的库和模块
use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

// 主要的表情包生成函数
fn christmas_gift(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    // 注意：当宽度和高度为0时，表示该帧不贴头像
    let locs = [
        (0, 0, 0, 0),      // 第0帧：贴头像
        (0, 0, 0, 0),      // 第1帧：贴头像
        (300, 215, 0, 244),    // 第2帧：贴头像
        (300, 215, 0, 197),    // 第3帧：贴头像
        (300, 215, 0, 159),    // 第4帧：贴头像
        (300, 215, 0, 129),    // 第5帧：贴头像
        (300, 215, 0, 105),    // 第6帧：贴头像
        (300, 215, 0, 87),     // 第7帧：贴头像
        (300, 215, 0, 74),     // 第8帧：贴头像
        (300, 215, 0, 67),     // 第9帧：贴头像
        (300, 215, 0, 61),     // 第10帧：贴头像
        (300, 215, 0, 57),     // 第11帧：贴头像
        (300, 215, 0, 53),     // 第12帧：贴头像
        (300, 215, 0, 51),     // 第13帧：贴头像
        (300, 215, 0, 49),     // 第14帧：贴头像
        (300, 215, 0, 48),     // 第15帧：贴头像
        (300, 215, 0, 46),     // 第16帧：贴头像
        (300, 215, 0, 48),     // 第17帧：贴头像
        (300, 215, 0, 51),     // 第18帧：贴头像
        (300, 215, 0, 56),     // 第19帧：贴头像
        (300, 215, 0, 62),     // 第20帧：贴头像
        (300, 215, 0, 66),     // 第21帧：贴头像
        (300, 215, 0, 67),     // 第22帧：贴头像
        (300, 215, 0, 67),     // 第23帧：贴头像
        (300, 215, 0, 67),     // 第24帧：贴头像
        (300, 215, 0, 65),     // 第25帧：贴头像
        (300, 215, 0, 65),     // 第26帧：贴头像
        (300, 215, 0, 61),     // 第27帧：贴头像
        (300, 215, 0, 60),     // 第28帧：贴头像
        (300, 215, 0, 60),     // 第29帧：贴头像
        (300, 215, 0, 60),     // 第30帧：贴头像
        (300, 215, 0, 67),     // 第31帧：贴头像
        (300, 215, 0, 92),     // 第32帧：贴头像
        (300, 215, 0, 141),    // 第33帧：贴头像
        (300, 215, 0, 201),    // 第34帧：贴头像
        (300, 215, 0, 245),    // 第35帧：贴头像
        (0, 0, 0, 0),          // 第36帧：不贴头像
        (0, 0, 0, 0),          // 第37帧：不贴头像
        (0, 0, 0, 0),          // 第38帧：不贴头像
        (0, 0, 0, 0),          // 第39帧：不贴头像
        (0, 0, 0, 0),          // 第40帧：不贴头像
        (0, 0, 0, 0),          // 第41帧：不贴头像
    ];

    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();

    // 循环生成42帧动画
    for i in 0..42 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];

        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("christmas_gift/{i}.png"))?;

        // 创建与帧图像相同尺寸的画布
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();

        // 清空画布为白色背景
        canvas.clear(Color::WHITE);

        // 只有当w和h不为0时才贴头像
        if w > 0 && h > 0 {
            // 调整输入图像到指定尺寸
            let image = image.resize_exact((w, h));
            // 在画布上绘制调整后的输入图像
            canvas.draw_image(&image, (x, y), None);
        }

        // 在画布上绘制咖波贴图（覆盖在输入图像上方）
        canvas.draw_image(&frame, (0, 0), None);

        // 将当前帧添加到GIF编码器，设置帧间隔为0.03秒
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }

    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

// 注册表情包插件
register_meme! {
    "christmas_gift",           // 表情包标识符
    christmas_gift,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["圣诞礼物"], // 搜索关键词
    date_created = local_date(2025, 12, 21),
    date_modified = local_date(2025, 12, 21),
}
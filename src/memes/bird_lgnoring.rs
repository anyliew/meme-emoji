// 导入所需的库和模块
use skia_safe::Image; // 图形库，用于颜色处理

use meme_generator_core::error::Error; // 表情包生成器核心错误类型
use meme_generator_utils::{
    builder::InputImage,                                      // 输入图像处理
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif}, // GIF 编码器
    image::ImageExt,                                          // 图像扩展功能
    tools::{load_image, local_date, new_surface}, // 工具函数：加载图像、本地日期、创建画布
};

use crate::{options::NoOptions, register_meme}; // 当前crate的模块

// 主要的表情包生成函数
fn bird_lgnoring(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    let params = [
        (55, 55, 95, 69),
        (55, 55, 95, 69),
        (55, 55, 95, 69),
        (55, 55, 95, 66),
        (55, 55, 95, 64),
        (55, 55, 95, 61),
        (55, 55, 95, 60),
        (55, 55, 96, 58),
        (55, 55, 96, 60),
        (55, 55, 96, 61),
        (55, 55, 96, 65),
        (55, 55, 96, 66),
        (55, 55, 95, 67),
        (55, 55, 95, 66),
        (55, 55, 95, 65),
        (55, 55, 95, 63),
        (55, 55, 96, 60),
        (55, 55, 96, 60),
        (55, 55, 96, 61),
        (55, 55, 96, 62),
        (55, 55, 96, 65),
        (55, 55, 96, 66),
        (55, 55, 96, 69),
        (55, 55, 96, 69),
        (55, 55, 95, 68),
        (55, 55, 95, 64),
        (55, 55, 95, 63),
        (55, 55, 95, 60),
        (55, 55, 95, 59),
        (55, 55, 95, 59),
        (55, 55, 95, 59),
        (55, 55, 96, 62),
        (55, 55, 96, 64),
        (55, 55, 96, 66),
        (55, 55, 96, 68),
        (55, 55, 96, 70),
        (55, 55, 95, 70),
        (55, 55, 95, 68),
        (55, 55, 95, 65),
        (55, 55, 96, 63),
        (55, 55, 96, 60),
        (55, 55, 96, 59),
        (55, 55, 96, 59),
        (55, 55, 96, 61),
        (55, 55, 96, 62),
        (55, 55, 96, 64),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let (w, h, x, y) = params[i];
        let frame = load_image(format!("bird_lgnoring/{i}.png"))?;
        let user_head = images[0].resize_exact((w, h));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 14,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

// 注册表情包插件
register_meme! {
    "bird_lgnoring",           // 表情包标识符
    bird_lgnoring,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["不鸟你","我鸟都不鸟你"], // 搜索关键词
    date_created = local_date(2025, 12, 24),
    date_modified = local_date(2025, 12, 24),
}

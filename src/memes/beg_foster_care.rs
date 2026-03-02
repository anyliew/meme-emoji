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
fn beg_foster_care(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (x坐标, y坐标)
    let params = [(72, 37); 14];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("beg_foster_care/{i}.png"))?;
        let user_head = images[0].resize_exact((146, 130));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, params[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 14,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

// 注册表情包插件
register_meme! {
    "beg_foster_care",           // 表情包标识符
    beg_foster_care,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["求包养"], // 搜索关键词
    date_created = local_date(2025, 10, 6),
    date_modified = local_date(2025, 10, 6),
}

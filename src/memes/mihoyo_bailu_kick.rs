// 导入所需的库和模块
use skia_safe::Color; // 图形库，用于颜色处理

use meme_generator_core::error::Error; // 表情包生成器核心错误类型
use meme_generator_utils::{
    builder::InputImage,     // 输入图像处理
    encoder::GifEncoder,     // GIF 编码器
    image::ImageExt,         // 图像扩展功能
    tools::{load_image, local_date, new_surface}, // 工具函数：加载图像、本地日期、创建画布
};

use crate::{options::NoOptions, register_meme}; // 当前crate的模块

// 主要的表情包生成函数
fn mihoyo_bailu_kick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    let locs = [
    (60, 60, 49, 67),
    (60, 60, 49, 67),
    (60, 60, 49, 67),
    (60, 60, 49, 67),
    (60, 60, 49, 67),
    (60, 60, 31, 35),
    (60, 60, 11, 32),
    (60, 60, 1, 47),
    (60, 60, 2, 60),
    (60, 60, 3, 62),
    (60, 60, 7, 64),
    (60, 60, 9, 70),
    (60, 60, 8, 76),
    (60, 60, 4, 106),
    (60, 60, 2, 157),
    (60, 60, 11, 156),
    (60, 60, 29, 47),
    (60, 60, 47, 1),
    (60, 60, 56, -28),
    (60, 60, 66, -26),
    (60, 60, 92, -36),
    (60, 60, 127, 4),
    (60, 60, 150, 56),
    (60, 60, 130, 63),
    (60, 60, 86, 46),
    (60, 60, 28, 54),
    (60, 60, -25, 151),
    (60, 60, 13, 99),
    (60, 60, 132, 29),
    (60, 60, 167, 6),
    (60, 60, 207, 1),
    (60, 60, 220, 29),
    (60, 60, 227, 61),
    (60, 60, 159, 17),
    (60, 60, 104, -28),
    (60, 60, 76, -39)
    ];
    
    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();
    
    // 循环生成数量帧动画
    for i in 0..36 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];
        
        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("mihoyo_bailu_kick/{i}.png"))?;
        
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
        
        // 将当前帧添加到GIF编码器，设置帧间隔为0.04秒
        encoder.add_frame(surface.image_snapshot(), 0.09)?;
    }
    
    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

// 注册表情包插件
register_meme! {
    "mihoyo_bailu_kick",           // 表情包标识符
    mihoyo_bailu_kick,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["白露踢"], // 搜索关键词
    date_created = local_date(2025, 10, 6), 
    date_modified = local_date(2025, 10, 6),
}
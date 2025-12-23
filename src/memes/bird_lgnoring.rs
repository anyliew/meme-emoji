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
fn bird_lgnoring(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    let locs = [
        (55, 55, 95, 69), (55, 55, 95, 69), (55, 55, 95, 69), (55, 55, 95, 66), (55, 55, 95, 64), (55, 55, 95, 61),
        (55, 55, 95, 60), (55, 55, 96, 58), (55, 55, 96, 60), (55, 55, 96, 61), (55, 55, 96, 65),
        (55, 55, 96, 66), (55, 55, 95, 67), (55, 55, 95, 66), (55, 55, 95, 65), (55, 55, 95, 63),
        (55, 55, 96, 60), (55, 55, 96, 60), (55, 55, 96, 61), (55, 55, 96, 62), (55, 55, 96, 65),
        (55, 55, 96, 66), (55, 55, 96, 69), (55, 55, 96, 69), (55, 55, 95, 68), (55, 55, 95, 64),
        (55, 55, 95, 63), (55, 55, 95, 60), (55, 55, 95, 59), (55, 55, 95, 59), (55, 55, 95, 59),
        (55, 55, 96, 62), (55, 55, 96, 64), (55, 55, 96, 66), (55, 55, 96, 68), (55, 55, 96, 70),
        (55, 55, 95, 70), (55, 55, 95, 68), (55, 55, 95, 65), (55, 55, 96, 63), (55, 55, 96, 60),
        (55, 55, 96, 59), (55, 55, 96, 59), (55, 55, 96, 61), (55, 55, 96, 62), (55, 55, 96, 64)
    ];
    
    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();
    
    // 循环生成数量帧动画
    for i in 0..46 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];
        
        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("bird_lgnoring/{i}.png"))?;
        
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
        
        // 将当前帧添加到GIF编码器，设置帧间隔为0.07秒
        encoder.add_frame(surface.image_snapshot(), 0.07)?;
    }
    
    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
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
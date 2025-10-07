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
fn doro_knight(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    let locs = [
    (133, 133, 126, 203),
    (133, 133, 126, 203),
    (133, 133, 125, 197),
    (133, 133, 125, 204),
    (133, 133, 125, 209),
    (133, 133, 128, 197),
    (133, 133, 125, 200),
    (133, 133, 124, 207),
    (133, 133, 125, 201),
    (133, 133, 125, 201),
    (133, 133, 125, 207),
    (133, 133, 125, 209),
    (133, 133, 126, 201),
    (133, 133, 125, 205),
    (133, 133, 125, 198),
    (133, 133, 127, 197),
    (133, 133, 127, 206)
    ];
    
    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();
    
    // 循环生成数量帧动画
    for i in 0..17 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];
        
        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("doro_knight/{i}.png"))?;
        
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
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    
    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

// 注册表情包插件
register_meme! {
    "doro_knight",           // 表情包标识符
    doro_knight,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["骑士","doro骑士","Doro骑士","DORO骑士"], // 搜索关键词
    date_created = local_date(2025, 10, 6), 
    date_modified = local_date(2025, 10, 6),
}
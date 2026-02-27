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
fn christmas_eve(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义三帧中头像的位置坐标 (x, y)
    let positions = [
        (70, 120), // 第1帧
        (70, 120), // 第2帧
        (70, 120), // 第3帧
    ];
    
    // 获取输入的第一张图像并调整到指定尺寸 (118, 89)
    let user_head = images[0].image.resize_exact((118, 89));

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();
    
    // 循环生成3帧动画
    for i in 0..3 {
        // 加载对应的背景帧
        let frame = load_image(format!("christmas_eve/{}.png", i + 1))?;
        
        // 创建与帧图像相同尺寸的画布
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        // 清空画布为透明背景（RGBA模式下）
        canvas.clear(Color::TRANSPARENT);
        
        // 获取当前帧的头像位置
        let (x, y) = positions[i];
        
        // 在画布上绘制用户头像
        canvas.draw_image(&user_head, (x, y), None);
        
        // 在画布上绘制背景帧（覆盖在头像上方）
        canvas.draw_image(&frame, (0, 0), None);
        
        // 将当前帧添加到GIF编码器，设置帧间隔为0.5秒
        encoder.add_frame(surface.image_snapshot(), 0.5)?;
    }
    
    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

// 注册表情包插件
register_meme! {
    "christmas_eve",      // 表情包标识符
    christmas_eve,        // 处理函数
    min_images = 1,       // 最少需要1张输入图片
    max_images = 1,       // 最多支持1张输入图片
    keywords = &["平安夜", "平安夜快乐"], // 搜索关键词
    date_created = local_date(2025, 8, 18), 
    date_modified = local_date(2025, 12, 9),
}
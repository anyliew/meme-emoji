// 导入所需的库和模块
use skia_safe::Color; // 图形库，用于颜色处理

use meme_generator_core::error::Error; // 表情包生成器核心错误类型
use meme_generator_utils::{
    builder::InputImage,     // 输入图像处理
    encoder::GifEncoder,     // GIF 编码器
    image::ImageExt,         // 图像扩展功能
    tools::{load_image, local_date, new_surface}, // 工具函数：加载图像、本地日期、创建画布
};

use crate::{options::NoOptions, register_meme, tags::MemeTags}; // 当前crate的模块

// 主要的表情包生成函数
fn kurogames_iuno_kick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    let locs = [
    (108, 108, 88, 40),   // 第0帧
    (108, 108, 88, 40),   // 第1帧
    (108, 108, 88, 40),   // 第2帧
    (105, 105, 122, 49),  // 第3帧
    (105, 105, 134, 54),  // 第4帧
    (105, 105, 142, 61),  // 第5帧
    (110, 110, 142, 69),  // 第6帧
    (105, 105, 129, 78),  // 第7帧
    (110, 110, 111, 78),  // 第8帧
    (110, 110, 96, 78),   // 第9帧
    (112, 112, 66, 70),   // 第10帧
    (108, 108, 29, 54),   // 第11帧
    (105, 105, 26, 47),   // 第12帧
    (96, 96, 29, 46),     // 第13帧
    (86, 86, 43, 39),     // 第14帧
    (70, 70, 75, 10),     // 第15帧
    (67, 67, 86, 0),      // 第16帧
    (61, 61, 90, -10),    // 第17帧
    (58, 58, 94, -16),    // 第18帧
    (62, 62, 92, -12),    // 第19帧
    (66, 66, 85, -4),     // 第20帧
    (77, 77, 59, 30),     // 第21帧
    (84, 84, 45, 41),     // 第22帧
    (97, 97, 29, 47),     // 第23帧
    (105, 105, 26, 48),   // 第24帧
    (110, 110, 42, 63),   // 第25帧
    (110, 110, 67, 71),   // 第26帧
    (110, 110, 96, 77),   // 第27帧
    (110, 110, 111, 78),  // 第28帧
    (105, 105, 137, 74),  // 第29帧
    (105, 105, 142, 69),  // 第30帧
    (105, 105, 141, 61),  // 第31帧
    (105, 105, 135, 54),  // 第32帧
    (105, 105, 102, 44),  // 第33帧
    (105, 105, 89, 39),  // 第33帧
    ];
    
    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();
    
    // 循环生成35帧动画
    for i in 0..35 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];
        
        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("kurogames_iuno_kick/{i}.png"))?;
        
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
        
        // 将当前帧添加到GIF编码器，设置帧间隔为0.06秒
        encoder.add_frame(surface.image_snapshot(), 0.06)?;
    }
    
    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

// 注册表情包插件
register_meme! {
    "kurogames_iuno_kick",           // 表情包标识符
    kurogames_iuno_kick,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["尤诺踢"], // 搜索关键词
    tags = MemeTags::capoo(),
    date_created = local_date(2025, 10, 6), // 创建日期：2022年11月29日
    date_modified = local_date(2025, 10, 6), // 修改日期：2023年2月14日
}
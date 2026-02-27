import os
import glob

def generate_mod_file():
    # 配置路径
    source_dir = r"D:\meme_rust\meme-emoji\src\memes"
    output_file = r"D:\meme_rust\meme-emoji\src\memes.rs"
    
    # 检查源目录是否存在
    if not os.path.exists(source_dir):
        print(f"错误：目录 {source_dir} 不存在")
        return
    
    # 获取所有.rs文件（不包括子目录）
    rs_files = glob.glob(os.path.join(source_dir, "*.rs"))
    
    # 提取文件名（不带路径和扩展名）
    mod_names = []
    for file_path in rs_files:
        filename = os.path.basename(file_path)
        if filename != "mod.rs":  # 排除mod.rs文件本身（如果有的话）
            mod_name = os.path.splitext(filename)[0]  # 去掉扩展名
            mod_names.append(mod_name)
    
    # 按字母顺序排序
    mod_names.sort()
    
    # 生成mod语句
    mod_lines = [f"mod {name};" for name in mod_names]
    
    # 写入目标文件
    try:
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write('\n'.join(mod_lines))
        
        print(f"成功生成 {len(mod_lines)} 个mod语句到 {output_file}")
        print("生成的内容：")
        for line in mod_lines:
            print(f"  {line}")
            
    except Exception as e:
        print(f"写入文件时出错：{e}")

if __name__ == "__main__":
    generate_mod_file()
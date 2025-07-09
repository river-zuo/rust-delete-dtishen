use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 目标目录，默认为当前目录
    #[arg(value_name = "DIR", default_value = ".")]
    dir: PathBuf,

    /// 一次性确认全部删除
    #[arg(long)]
    all: bool,

    /// 指定要删除的文件或目录的后缀名，默认为“的替身”
    #[arg(long, default_value = "的替身")]
    suffix: String,
}

fn main() {
    let args = Args::parse();
    println!("扫描目录: {}，后缀: {}", args.dir.display(), args.suffix);
    let mut targets = Vec::new();
    find_targets(&args.dir, &mut targets, &args.suffix);
    if targets.is_empty() {
        println!("未找到以'{}'结尾的文件或目录。", args.suffix);
        return;
    }
    if args.all {
        println!("找到以下以'{}'结尾的文件或目录:", args.suffix);
        for path in &targets {
            println!("  {}", path.display());
        }
        print!("是否全部删除以上 {} 个文件/目录? [y/N] ", targets.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        if input == "y" || input == "yes" {
            for path in targets {
                let path_str = path.display();
                if path.is_dir() {
                    match fs::remove_dir_all(&path) {
                        Ok(_) => println!("已删除目录: {}", path_str),
                        Err(e) => eprintln!("删除目录失败: {}: {}", path_str, e),
                    }
                } else {
                    match fs::remove_file(&path) {
                        Ok(_) => println!("已删除文件: {}", path_str),
                        Err(e) => eprintln!("删除文件失败: {}: {}", path_str, e),
                    }
                }
            }
        } else {
            println!("未删除任何文件/目录。");
        }
    } else {
        for path in targets {
            let path_str = path.display();
            print!("是否删除: {}? [y/N] ", path_str);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "y" || input == "yes" {
                if path.is_dir() {
                    match fs::remove_dir_all(&path) {
                        Ok(_) => println!("已删除目录: {}", path_str),
                        Err(e) => eprintln!("删除目录失败: {}: {}", path_str, e),
                    }
                } else {
                    match fs::remove_file(&path) {
                        Ok(_) => println!("已删除文件: {}", path_str),
                        Err(e) => eprintln!("删除文件失败: {}: {}", path_str, e),
                    }
                }
            } else {
                println!("跳过: {}", path_str);
            }
        }
    }
}

fn find_targets(dir: &Path, targets: &mut Vec<PathBuf>, suffix: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file_name.ends_with(suffix) {
                targets.push(path.clone());
            }
            if path.is_dir() {
                find_targets(&path, targets, suffix);
            }
        }
    }
} 
// 使用内置的 env 模块来获取命令行参数，由于只使用args方法，所以将args方法留给变量，在代码中使用env::args()来调用
use std::env;
// 系统控制
use std::process;
// 引入库中定义的结构体
use minigrep::Array;
use minigrep::CommandConfig;


/**
 * 用法是 minigrep 关键字 文件路径
 */
fn main() {
    // 获取命令行参数
    let args: Array = Array(env::args().collect());
    println!("You tab commands: {}\n", args);
    let config = CommandConfig::parse(&args.0).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("query: {1}, file_path: {0}\n", config.query, config.file_path);

    // 读取并搜索文件内容，由于在此处我们并不关注search的Ok返回值（因为所有的逻辑都在search中处理），这用march的if let实现
    if let Err(e) = minigrep::search(&config){
        println!("Problem reading file {} : {e}", config.file_path);
        process::exit(2);
    }
}
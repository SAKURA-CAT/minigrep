// 使用内置的 env 模块来获取命令行参数，由于只使用args方法，所以将args方法留给变量，在代码中使用env::args()来调用
use std::env;
// 输出可视化
use std::fmt;
// 文件读取
use std::fs;
use std::string;


/**
 * Array实际上是包装了Vec，主要拓展了Display功能
 */
struct Array(Vec<String>);

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push('[');
        for i in &self.0 {
            s.push_str(&i);
            if i != &self.0[self.0.len()-1]{
                s.push_str(", ");
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}


fn main() {
    // 获取命令行参数
    let args: Array = Array(env::args().collect());
    println!("{}", args);
    let config = parse_command(&args.0);
    println!("query: {1}, file_path: {0}", config.query, config.file_path);
    // 读取文件内容,file_path的生命在此结束
    let conents = fs::read_to_string(config.file_path).expect("");
    println!("Content: \n{conents}")
}



/**
 * 解析命令行参数，用法是 minigrep 关键字 文件路径，所以索引1是关键字，索引2是文件路径
 * 
 * # 参数
 * args: 命令行参数，此时如果输入的是&Vec<String>，会发生从字符串变长数组到字符串切片的引用的隐式变换
 * 
 * # 返回值
 * (query, file_path)
 * query: 关键字
 * file_path: 文件路径
 * 
 * # 用法
 */
fn parse_command(args: &[String])-> CommandConfig{
    let query = args[1].clone();
    let file_path = args[2].clone();
    // 返回处理后的数组
    CommandConfig { query, file_path}
}


/**
 * 解析命令行后，返回命令行配置
 * 
 * # 参数
 * query: 寻找的关键字
 * file_path: 文件路径
 */
 struct CommandConfig{
    query: String,
    file_path: String
 }





// 输出可视化
use std::fmt;
// 文件读取
use std::fs;
// 错误捕获
use std::error::Error;


/**
 * Array实际上是包装了Vec，主要拓展了Display功能
 */
pub struct Array(pub Vec<String>);

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

/**
 * 解析命令行后，返回命令行配置
 * 
 * # 参数
 * query: 寻找的关键字
 * file_path: 文件路径
 */
 pub struct CommandConfig{
    pub query: String,
    pub file_path: String
 }


impl CommandConfig{
    /**
     * 传入env::args，创建一个命令配置，索引0是关键字，索引1是文件路径
     * 此方法没有添加错误检查，需使用parse方法
     * 
     * # 参数
     * args: 字符串数组
     * 
     * # 返回值
     * (query, file_path)
     * query: 关键字
     * file_path: 文件路径
     * 
     * # 用法
     */
    fn new(commands: &[String]) -> CommandConfig{
        // 错误处理，
        let query = commands[0].clone();
        let file_path = commands[1].clone();
        // 返回处理后的数组
        CommandConfig { query, file_path}
    }

    /**
     * 解析命令，返回一个命令配置，添加了错误捕获
     * 由于标准库会将命令本身也当作命令行解析的一部分，所以第一个索引并不算做处理的一部分，在这个函数中被切片
     * 
     * # 参数
     * args: 命令行参数，此时如果输入的是&Vec<String>，会发生从字符串变长数组到字符串切片的引用的隐式变换
     * 
     * # 返回值
     * 返回值是一个Result，第一个参数是CommandConfig，第二个参数是静态编译字符串
     * 可以给静态字符串增加生命周期标识，str生命周期将与整个程序的生命周期相同（直到程序结束才被释放）
     */
    pub fn parse(args: &[String]) -> Result<CommandConfig, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        } 
        // 没有任何问题
        Ok(CommandConfig::new(&args[1..3]))
    }
}



/**
 * 输入查询的文件路径和查询的关键字，进行查询，实际上没有对输入的内容做更改，因此这里采用借用的形式
 * 因为要实现错误捕捉，所以需要使用Result结构体，需要满足Result<T,E> 的要求，因此使用了 Ok(()) 返回一个单元类型 ()
 * Box<dyn Error>说明这是一个实现了Error特征的特征对象，这样我们就无需指定具体的错误类型
 */
pub fn search(config: &CommandConfig) -> Result<(), Box<dyn Error>>{
    // 借用的方式，所以需要借用file_path的所有权
    let contents = fs::read_to_string(&config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}


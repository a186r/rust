// use std::env;
// use std::fs::File;
// use std::io::prelude::*;
// use std::process;
// use std::error::Error;
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main(){
    // 将命令行参数收集到一个vector中，并且打印出来
    // 这里的args变量是参数值的所有者并只允许parse_config借用他们，
    // 如果Config尝试获取args中值的所有权会违反Rust的借用规则
    // let args: Vec<String> = env::args().collect();
    
    // let query = &args[1];
    // let filename = &args[2];

    // let config = parse_config(&args);
    
    // let config = Config::new(&args);

    // 处理错误情况
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     // println!("Problem parsing arguments: {}", err);
    //     // eprintln!("Application error: {}",e);
    //     process::exit(1);
    // });

// 将env的返回值传递给Config::new
// env::args返回一个迭代器，不同于将迭代器的值收集到一个vector中接着传递一个
// slice给Config::new，我们直接将env::args返回的迭代器的所有权传递给Config::new.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });

    // println!("Searching for {} " ,config.query);
    // println!("In file {}",config.filename);

    // 提取一个run函数
    // run(config);
    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {}",e);

        process::exit(1);
    }
//     let mut f = File::open(config.filename).expect("file not found");
//     let mut contents = String::new();

// // 读取文件中的内容
//     f.read_to_string(&mut contents)
//         .expect("something went wrong reading the file");

//         println!("With text:\n{}",contents);
}

// -------------------------------------------------------------二进制项目的关注分离
// 1.将程序拆分成main.rs和lib.rs并将程序的逻辑放入lib.rs中。
// 2.当命令行解析逻辑比较小时，卡伊保留在main.rs中。
// 3.当命令行解析开始变得复杂时，也同样将其从main.rs提取到lib.rs中。
// 4.如果run返回错误，则处理这个错误。

// main程序负责处理程序运行，而lib.rs处理所有的真正的任务逻辑

// struct Config{
//     query: String,
//     filename: String,
// }

// 直接返回一个结构体
// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1].clone();
//     let filename = &args[2].clone();

//     Config{query,filename}
// }

// 创建一个Config构造函数
// impl Config{
//     // 现在new返回的是一个Result，成功是带有一个Config实例而在出现错误时带有一个&'static str
//     // &'static str是字符串字面值的类型，也是目前俄错误信息
//     fn new(args: &[String]) -> Result<Config,&'static str> {
//         // 增加一个检查，在访问索引1和2之前检查slice是否足够长，如果不够，返回一个错误信息panic
//         if args.len() < 3 {
//             // 这里虽然返回了合理的错误信息，但是panic!的调用更趋向于是程序的问题而不是使用上的问题
//             // 可以使用Result类型来表明这里存在的问题
//             // panic!("not enough arguments");
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config {query,filename})
//     }
// }

// 现在run函数包含了main中从读取文件看是的剩余的所有逻辑。run函数获取一个config实例作为参数。

// 修改run函数返回Result
// fn run(config: Config) -> Result<(),Box<Error>> {
//     // let mut f = File::open(config.filename).expect("file not found");
//     let mut f = File::open(config.filename)?;

//     let mut contents = String::new();

// // 读取文件中的内容
//     // f.read_to_string(&mut contents)
//         // .expect("something went wrong reading the file");
//     f.read_to_string(&mut contents)?;

//     println!("With text:\n{}",contents);
//     Ok(())
// }
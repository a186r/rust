// 健身的例子
use std::thread;
use std::time::Duration;

fn main() {
    // println!("这是推荐的xxxx: {}",simulated_expensive_calculation(2));
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

// 调用generate_workout函数
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

// 现在逻辑有了，开始编写具体实现,u32是占32个字节的无符号整数，有符号整数类型以i开头
// 现在使用函数重构这个方法
// 然后重构，使用闭包存储代码

// 在竖线中指定闭包的参数，参数之后是存放闭包体的大括号，如果闭包体只有一行，那之后的大括号是可以省略的
let expensive_closure = |num| {
    println!("calculation slowly...")；
    thread::sleep(Duration::from_secs(2));
    num
}

fn generate_workout(intensity: u32,random_number: u32){

    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25{
        println!(
            "今天，做 {} 个俯卧撑",
            // simulated_expensive_calculation(intensity)
            expensive_result
        );

        println!(
            "然后，做 {} 个深蹲起",
            // simulated_expensive_calculation(intensity)
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("今天休息以下，多喝热水，哈哈啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊");
        } else {
            println!(
                "今天，跑步 {} 分钟",
                // simulated_expensive_calculation(intensity)
                expensive_result
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    // 休息两秒
    thread::sleep(Duration::from_secs(2));
    intensity
}
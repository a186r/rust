use std::thread;
use std::time::Duration;

fn main() {
    // let get_return = simulated_expensive_calculation(12);
    // println!("get return: {}",get_return);
    let simulated_user_pecified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_pecified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32,random_number: u32){
    // 将重复的函数提取到一个变量中
    // let expensive_result = 
    //     simulated_expensive_calculation(intensity);

    // 但是现在所有的情况下都需要调用函数并等待结果，
    // 我们希望在程序的一个位置指定某些代码，并在程序的某处实际需要结果的时候执行这些代码
    // 这正是闭包的用武之地

    // 使用闭包存储代码

    // 如果闭包有多个参数，用逗号分隔 |a,v,b,c|，参数之后是存放闭包的大括号，闭包体的最后
    // 一行的返回的值将是调用闭包时返回的值
    // 调用闭包类似调用函数
    // let expensive_closure = |num :u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today,do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remeber to stay hydrated!");
        }else{
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }


// 定义一个 Cacher 结构体来在 calculation 中存放闭包并在 value 中存放 Option 值
struct Cacher<T>
    where T: Fn(u32) -> u32  //T是使用Fn的闭包
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher <T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value:None,
        }
    }

    fn value(&mut self,arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[test]
fn call_with_different_calues(){
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2,2);
}

#[test]
fn equal_to(){
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
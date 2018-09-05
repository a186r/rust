fn main() {
    // let mut x = 5;
    // println!("The value of x is : {}",x);
    // x = 6;
    // println!("The value of x is : {}",x);

    // 声明常量
    // const MAX_POINTS: u32 = 100_000;

// shadowing
    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is : {}",x);
    // let spaces = "         ";
    // let spaces = spaces.len();
    // println!("The spaces is : {}",spaces);

// 必须添加类型注解
    // let guess: u32 = "32".parse().expect("Not a number!");
    // println!("guess is : {}",guess);

// // 浮点型
// // f64
//     let x = 2.0;

// // f32
//     let y: f32 = 3.0;

    // 元组
    // let tup: (i32,f64,u8) = (500,3.2,1);

// 通过模式匹配来解构元组
    // let tup = (500,3.5,1);
    // let (x,y,z) = tup;
    // println!("The value of y is: {}",y)

    // let x: (i32,f64,u8) = (500,4.2,3);

    // let five_handred = x.0;
    // let four_point_two = x.1;
    // let three = x.2;

    // 数组
    // let a = [1,2,3];
    // another_function(11,12);

//     let x = 5;

// // 表达式的结尾没有分号，如果加上分号这就变成了语句，语句不会返回值
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is : {}",y)

    // let y = five();
    // println!("The value of y is : {}",y);
    
    let t = plus_one(3);
    println!("The value of y is : {}",t);
}

// 必须声明每一个参数的类型，使用逗号分隔
// fn another_function(x: i32,y: i32){
//     println!("The value of x is: {}",x);
//     println!("The value of y is: {}",y);
// }

    // 函数的返回值等同于函数体最后一个表达式的值，不对返回值命名，但是要在剪头后声明它的类型。

// fn five() -> i32{
//     let x = 9;
//     x * 3
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}
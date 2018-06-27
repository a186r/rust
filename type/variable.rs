// fn main() {
//     // 声明变量类型
//     let logical: bool = true;

//     let a_float: f64 = 1.0; //常规声明
//     let an_integer = 5i32; //后缀声明

//     // 自动推断类型
//     let default_float = 3.0;
//     let default_integer = 8;

//     let mut mutable = 12;

//     // mutable = true;//变量的类型不可改变
// }

fn main() {
    // u32表示32位存储的无符号整数，i32表示32位存储的带符号整数
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    // 使用下划线改善数字的可读性
    println!("One million is written as {}", 1_000_000u32);
}

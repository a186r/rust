// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}" , s1 ,len);
// }

// // 传入一个引用的String
// // &允许你获取值而不是String
// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String){
//     some_string.push_str(",world");
// }

// 在特定作用域中的特定数据有且只有一个可变引用
// 所以可以使用大括号来创建一个新的作用域来允许拥有多个可变引用，只是不能同时拥有
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     }

//     let r2 = &mut s;
// }

// 不能在拥有不可变引用的同时拥有可变引用,但是如果每个都在自己的作用域就可以
// fn main() {
//     let mut s = String::from("hello");

//     {let r1 = &s;}

//     {let r2 = &s;}

//     {
//         let r3 = &mut s;
//     }
// }

// 悬垂引用，mmp哦
// 这里应该直接返回一个String，将所有权也移动出去，这样s就不会被释放了
// fn main() {
//     let reference_to_nothing = dangle();
// }

// // s被释放
// // fn dangle() -> &String{
// //     let s = String::from("hello");

// //     &s
// // }
// // 拿走所有权，s不会被释放
// fn dangle() -> String{
//     let s = String::from("hello");
//     s
// }
// mod outermost{
//     pub fn middle_function(){}

//     pub fn middle_secret_function(){}

//     pub mod inside {
//         pub fn inner_function(){
//             ::outermost::middle_secret_function();
//         }

//         pub fn secret_function(){}
//     }
// }

// fn try_me(){
//     outermost::middle_function();
//     outermost::middle_secret_function();
//     outermost::inside::inner_function();
//     outermost::inside::secret_function();
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// pub mod a{
//     pub mod series{
//         pub mod of{
//             pub fn nested_modules(){}
//         }
//     }
// }

// 可以将模块引入作用域
// use a::series::of;
// 也可以将函数本身引入,通过use指定函数，这使得我们可以忽略所有的模块直接引用函数
// use a::series::of::nested_modules;

// pub fn main(){
//     // of::nested_modules();
//     nested_modules();
// }

// 因为枚举也像模块一样组成了某种命名空间，也可以使用use来导入枚举的成员。
// 对于任何类型的use语句，如果一个命名空间导入多个项，可以在最后使用大括号和逗号来列举它们
// enum TrafficLight{
//     Red,
//     Yellow,
//     Green,
// }

// use TrafficLight::{Red,Yellow};

// fn main(){
//     let red = Red;
//     let yellow = Yellow;
//     // 我们仍然为Green成员指定了TrafficLight命名空间，因为并没有在use语句中包含Green。
//     let green = TrafficLight::Green;
// }

// -----------------------------------使用glob将所有名称引入作用域
// 为了一次将某个命名空间下所有名称都引入作用域，可以使用*,这称为glob运算符
// 请保守的使用*,虽然很方便，但是也可能会引入多于预期的内容从而导致命名冲突。
// enum TrafficLight {
//     Red，
//     Yellow,
//     Green,
// }

// use TrafficLight::*;

// fn main(){
//     let red = Red;
//     let yellow = Yellow;
//     let green = Green;
// }

// ----------------------------------使用super访问父模块
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(IpAddrKind::V4)
// }

// fn route(ip_type: IpAddrKind){}

// enum IPAddrKind{
//     V4,
//     V6,
// }

// // 使用了一个结构体将kind和address打包在一起，现在枚举成员就与值关联了
// struct IpAddr{
//     // kind字段是枚举类型的
//     kind:IPAddrKind.
//     // address字段是String类型的
//     address:String,
// }

// let home = IPAddr{
//     kind:IPAddrKind::V4,
//     address:String::from("127.0.0.1");
// };

// let loopback = IPAddr{
//     kind:IPAddrKind::V6,
//     address:String::from("::1"),
// };

// 可以以更简洁的方式来表达相同的概念
// 直接使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
// 直接将数据附加到每个枚举的成员上，这样就不需要一个额外的结构体了
// enum IpAddr{
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));
// let loopback = IpAddr::V6(String::from("::1"));

// 使用枚举处理结构体还有另一个优势，每个成员可以处理不同类型和数量的数据。
// enum IpAddr{
//     V4(u8,u8,u8,u8),
//     V6(String),
// }

// let home = IpAddr::V4(127,0,0,1);
// let loopback = IpAddr::V6(String::from("::1")));

// 储存ip这种情况太常见了，标准库提供了一个开箱即用的定义，可以将任意类型的数据放入枚举
// 成员中，例如字符串、数字类型或者结构体，甚至可以包含另一个枚举
// struct IpV4Addr{

// }

// struct IpV6Addr{

// }

// enum IpAddr{
//     V4(IpV4Addr),
//     V6(IpV6Addr),
// }

// 结构体和枚举还有另一个相似点，就像impl可以为结构体定义方法那样，我们也可以在枚举上定义方法
// 下面是定义一个在Message枚举上的叫做call的方法
// impl Message{
//     fn call(&self){

//     }
// }

// let m = Message::Write(String::from("hello"));
// m.call();

// 标准库中另一个非常常见且实用的枚举：Option
// Option类型应用广泛，因为它编码了一个非常普遍的场景，既一个值要么是某个值，要么什么都不是
// 从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他
// 编程语言中非常常见的bug

// enum OPtion<T>{
//     Some(T),
//     None,
// }

// let some_number = Some(5);
// let some_string = Some("a string");
// // 如果使用None而不是Some，需要告诉Rust Option<T>是什么类型的，因为编译器只通过None值无法推断出Some变量保留的值的类型
// let absent_number: Option<i32> = None;
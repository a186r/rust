// 使用trait定义一个特征
// trait HasArea {
//     fn area(&self) -> f64;
// }

// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl HasArea for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }

// fn main() {
//     let c = Circle {
//         x: 0.0f64,
//         y: 0.0f64,
//         radius: 1.0f64,
//     };

//     println!("circle c has an area of {}", c.area());
// }

// 泛型的trait约束
// use std::fmt::Debug;
// fn foo<T: Debug>(s: T) {
//     println!("{:?}", s);
// }
// 可以使用多个trait对泛型进行约束
// use std::fmt::Debug;
// fn foo<T: Debug + Clone>(s: T) {
//     s.clone();
//     println!("{:?}", s);
// }

// use std::fmt::Debug;
// fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
//     x.clone();
//     y.clone();
//     println!("{:?}", y);
// }

// // where从句
// fn foo<T, K>(x: T, y: K)
// where
//     T: clone,
//     K: Clone + Debug,
// {
//     x.clone();
//     y.clone();
//     println!("{:?}", y);
// }

// // 或者
// fn foo<T, K>(x: T, y: K)
// where
//     T: Clone,
//     K: Clone + Debug,
// {
//     x.clone();
//     y.clone();
//     println!("{:?}", y);
// }

// // trait与内置类型
// trait HasArea {
//     fn area(&self) -> f64;
// }

// impl HasArea for i32{
//     fn area(&self) -> f64{
//         *self as f64
//     }
// }

// 5.area();

// // trait的默认方法
// trait Foo{
//     fn is_valid(&self -> bool);

//     fn is_invalid(&self) -> bool{!self.is_valid()}
// }

// // trait的继承
// trait Foo{
//     fn foo(&self);
// }

// trait FooBar:Foo{
//     fn foobar(&self);
// }

// // 这样FooBar的实现者也要同时实现Foo
// struct Baz;

// impl Foo for Baz {
//     fn foo(&self){
//         println!("foo");
//     }
// }

// impl FooBar for Baz{
//     fn foobar(&self){
//         println!("foobar");
//     }
// }

// trait对象
// trait对象在rust中是指使用指针封装了的trait，比如&SomeTrait和Box<SomeTrait>。

// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Foo for String {
//     fn method(&self) -> String {
//         format!("string: {}", *self)
//     }
// }

// fn do_something(x: &Foo) {
//     x.method();
// }

// fn main() {
//     let x = "Hello".to_string();
//     do_something(&x);

//     let y = 8u8;
//     do_something(&y);
// }

// trait对象的实现

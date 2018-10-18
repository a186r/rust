// fn main() {}

// fn call_with_one<F>(some_closure: F) -> i32
//     where F: Fn(i32) -> i32{

//     some_closure(2)

// }

// let answer = call_with_one(|x| x+2);

// assert_eq!(4,answer);
// fn main() {
//     fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
//         some_closure(1)
//     }

//     fn add_one(i: i32) -> i32 {
//         i + 1
//     }

//     let answer = call_with_one(&add_one);

//     assert_eq!(2, answer);
// }

// 使用box
// fn factory() -> Box<Fn(i32) -> i32> {
//     let num = 5;

//     Box::new(move |x| x + num)
// }

// fn main() {
//     let f = factory();

//     let answer = f(1);
//     assert_eq!(6, answer);
// }

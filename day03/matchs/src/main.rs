// // match分支和模式匹配

// // 一个枚举和一个以枚举成员作为模式的match表达式
// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u32{
//     match coin {
//         // 一个分支有两个部分：一个模式和一些代码
//         // 第一个分支的模式是Coin::Penny , =>将模式和代码分开 ， 这里的代码仅仅是1
//         // 每一个分支之间用逗号分隔
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


// ------------------------------### 绑定值的模式
// #[derive(Debug)]
// enum UsState{
//     Alabama,
//     Alaska,
// }

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin{
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!(
//                 "State quarter from {:?}!",
//                 state
//             );
//             25
//         },
//     }
// }

// ------------------------------------匹配Option<T>
// fn puls_one(x: Option<i32>) -> Option<i32> {
//     match x{
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// let five = Some(5);
// let six = puls_one(5);
// let none = puls_one(None);

// -----------------------------------匹配是穷尽的
// 如果我们没有处理None的情况，编译的时候就会报错，Rust知道我们没有覆盖所有可能的情况甚至知道哪些模式被忘记了
// Rust中的匹配是穷尽的：也就是说必须穷举到最后的可能性来使代码有效。
// fn puls_one(x: Option<i32>) -> Option<i32>{
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

// ----------------------------------通配符
// rust也提供了一个模式，用于不想列举出所有可能值的场景
//  _会匹配所有的值，将其放置于其他分支之后，_将会匹配所有之前没有指定的可能的值。()就是uint值，所以_的情况什么也不会发生
// 因此可以说我们想要对_通配符之前没有列出的所有可能的值不做任何处理。
// 但是match在只关心一个情况的场景中就有点啰嗦了，为此Rust提供了if let。
// let some_u8_value = 0u8;
// match some_u8_value{
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (),
// }
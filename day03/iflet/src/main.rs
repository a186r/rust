// --------------------------if let
// if let语法让我们以一种不那么冗长的方式结合if和let，来处理只匹配一个模式的值而忽略其他模式的情况
// 下面匹配一个Option<u8>值，并且只希望值为三时执行代码：
// let some_u8_value = Some(0u8);

// match some_u8_value {
//     Some(3) => println!("three"),
//     _ => (),
// }

// 我们可以使用if let这种更短的方式编写
// if let Some(3) = some_u8_value{
//     println!("three");
// }

// let mut count = 0;
// match coin{
//     Coin::Quarter(state) => println!("State quarter from {:?}!",state),
//     _ => count += 1,
// }

// 或者可以使用这样的if let和else表达式
let mut count = 0;

if let Coin::Quarter(state) = coin{
    println!(
        "State quarter from {:?}!",
        state
    );
}else{
    count += 1;
}
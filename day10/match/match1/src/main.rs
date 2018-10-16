// match初探
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let d_panic = Direction::North;
//     let d_west = Direction::East;

//     let d_str = match d_west {
//         Direction::East => "East",
//         Direction::North | Direction::South => {
//             panic!("South or North");
//         }
//         _ => "West",
//     };

//     println!("{}", d_str);
// }

// // 使用match解构数据

// enum Action {
//     Say(String),
//     MoveTo(i32, i32),
//     ChangeColorRGB(u16, u16, u16),
// }

// fn main() {
//     let action = Action::Say("Hello Rust".to_string());

//     match action {
//         Action::Say(s) => {
//             println!("{}", s);
//         }
//         Action::MoveTo(x, y) => {
//             println!("point from (0,0) move to ({},{})", x, y);
//         }
//         Action::ChangeColorRGB(r, g, _) => {
//             (
//                 "change color into '(r:{}, g:{}, b:0)','b' has been ignored",
//                 r,
//                 g,
//             );
//         }
//     }
// }

// 模式
// rust中模式可以被用来对任何符合类型进行解构，比如对struct进行解构
// struct Point {
//     x: i64,
//     y: i64,
// }

// fn main() {
//     // let point = Point { x: 0, y: 0 };
//     // match point {
//     //     Point { y: y1, .. } => println!("({})", y1),
//     // }

//     // let tuple: (u32, String) = (5, String::from("five"));
//     // let (x, s) = tuple;
//     // println!("Tuple is: {:?}", tuple);编译会报错，因为String类型没有实现Copy,所以Tuple倍整体Move掉了。

//     let tuple = (5, String::from("five"));
//     let (x, _) = tuple; //忽略String类型，而u32实现了Copy，则tuple不会被Move掉
//     println!("Tuple is {:?}", tuple);
// }

// 在模式匹配中，当我想要匹配一个数字范围的时候，可以用...来表示
// fn main() {
//     let x = 1;

//     match x {
//         1...10 => println!("一到十"),
//         _ => println!("其他"),
//     }

//     let c = 'w';

//     match c {
//         'a'...'z' => println!("小写字母{}", c),
//         'A'...'Z' => println!("大写字母{}", c),
//         _ => println!("其他字符"),
//     }

//     // 多重匹配
//     let y = 1;

//     match y {
//         1 | 2 => println!("1 or 2"),
//         _ => println!("其他数字"),
//     }

//     // ref 或 ref mut
//     // 当被模式匹配命中的时候，未实现Copy的类型会被Move掉，因此原owner就不再持有其所有权。
//     // 但是有些时候，我们只需要从中拿一个变量的(可变)引用，而不想将其move出作用域，这时候就可以用ref 或者 ref mut
//     let mut z = 5;
//     match z {
//         ref mut mr => println!("mut ref :{}", mr),
//     }

//     let ref mut mrx = z;
// }

// 变量绑定
// 在模式匹配的过程内部，我们可以用@来绑定一个变量名，这在复杂的模式匹配中再方便不过。
fn main() {
    // let x = 2u32;
    // match x {
    //     e @ 1...5 | e @ 10...15 => println!("get:{}", e),
    //     _ => (),
    // }
    // 如上，e绑定了x的值

    // 变量绑定是个及其有用的语法，下面是官方doc里的例子
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person {
            name: ref a @ Some(_),
            ..
        }) => println!("{:?}", a),
        _ => {}
    }
}

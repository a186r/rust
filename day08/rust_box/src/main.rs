// // 最简单直接的智能指针是box，类型是Box<T>
// // box允许你将一个值放在堆上而不是栈上
// // 正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放。
// // 这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。
// use List::{Cons,Nil};

// fn main(){
//     let b = Box::new(5);
//     println!("b = {}",b);
//     // 第一个Cons存储了1个另一个List值，以此类推
//     // let list = Cons(1,Cons(2,Cons(3,Nil)));
//     // list最多需要一个i32和一个Box的空间
//     let list = Cons(
//         1,Box::new(Cons(
//             2,Box::new(Cons(
//                 3,Box::new(
//                     Nil
//                 )
//             ))
//         ))
//     );
// }

// // rust需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是递归类型，其值的一部分可以使相同类型的另一个值

// enum List{
//     Cons(i32,Box<List>),
//     Nil,
// }

// // 回忆一下第六章的Message枚举
// // rust可以计算出Message需要多少空间，但是上面的List就不知道了，因为是递归的
// // enum Message{
// //     Quit,
// //     Move{x: i32,y: i32},
// //     Write(String),
// //     ChangeColor(i32,i32,i32),
// // }

fn main(){
    let x = 5;
    let y = &x;

    assert_eq!(5,x);
    assert_eq!(5,*y);
}
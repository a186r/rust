// 重写函数Add
// use std::ops::Add;

// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     println!("{}", add(100i32, 1i32));
//     println!("{}", add(100.11f32, 100.22f32));
// }

// 为自己自定义的数据类型实现add操作
// use std::ops::Add;

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// // 为Point实现Add trait
// impl Add for Point {
//     type Output = Point; //返回值类型为Point

//     fn add(self, p: Point) -> Point {
//         Point {
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     println!("{}", add(100i32, 1i32));
//     println!("{}", add(100.11f32, 2.22f32));

//     let p1 = Point { x: 1, y: 1 };

//     let p2 = Point { x: 2, y: 2 };
//     println!("{:?}", add(p1, p2));
// }

// 下面的例子不仅让自定义的Point类型支持了add操作，同时也为Point做了泛型化。
// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     x: T,
//     y: T,
// }

// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, p: Point<T>) -> Point<T> {
//         Point {
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     let p1 = Point {
//         x: 1.1f32,
//         y: 1.1f32,
//     };

//     let p2 = Point {
//         x: 2.1f32,
//         y: 2.22f32,
//     };

//     println!("{:?}", add(p1, p2));

//     let p3 = Point { x: 1i32, y: 1i32 };
//     let p4 = Point { x: 2i32, y: 2i32 };
//     println!("{:?}", add(p3, p4));
// }
// fn main() {
// let a: String = String::from("xyz");
// let b = a.clone(); //直接调用String的clone()方法，实现对内存的值拷贝而不是简单的地址拷贝。
// println!("{}", a);

// let a = vec![1, 2, 3];
// let mut a = a;
// a.push(4);
// println!("{:?}", a);

// let mut a: &str = "adbc";
// a = "xyz";
// println!("{}", a);
// }

// 如何实现Copy特性呢，一，通过derive让编译器自动实现
#[derive(Clone, Copy)]
struct Foo1 {
    a: i32,
    b: bool,
}
// 编译器 会自动检查Foo的所有属性是否实现了Copy特性，一旦检查通过，便会为Foo自动实现Copy特性。

// 二，手动实现
#[derive(Debug)]
struct Foo {
    a: i32,
    b: bool,
}

impl Copy for Foo {}
impl Clone for Foo {
    fn clone(&self) -> Foo {
        Foo {
            a: self.a,
            b: self.b,
        }
    }
}

fn main() {
    let x = Foo { a: 100, b: true };

    let mut y = x;

    y.b = false;

    println!("{:?}", x);
    println!("{:?}", y);
}

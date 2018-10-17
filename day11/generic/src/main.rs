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
// #[derive(Clone, Copy)]
// struct Foo1 {
//     a: i32,
//     b: bool,
// }
// // 编译器 会自动检查Foo的所有属性是否实现了Copy特性，一旦检查通过，便会为Foo自动实现Copy特性。

// // 二，手动实现
// #[derive(Debug)]
// struct Foo {
//     a: i32,
//     b: bool,
// }

// impl Copy for Foo {}
// impl Clone for Foo {
//     fn clone(&self) -> Foo {
//         Foo {
//             a: self.a,
//             b: self.b,
//         }
//     }
// }

// fn main() {
//     let x = Foo { a: 100, b: true };

//     let mut y = x;

//     y.b = false;

//     println!("{:?}", x);
//     println!("{:?}", y);
// }

// move关键字
// move关键字常用在闭包中，强制闭包获取所有权

// fn main() {
//     let x: i32 = 100;
//     let some_closure = move |i: i32| i + x; //闭包some_closure
//     let y = some_closure(2);
//     println!("x={},y={}", x, y);
// }

// 下面的例子会报错，因为move关键字，会把闭包中的外部变量的所有权move到包体内，发生了所有权转移的问题，所以println访问x会报错 error[E0382]: use of moved value: `x`
// fn main() {
//     let mut x: String = String::from("abc");
//     let mut some_closure = move |c: char| x.push(c);
//     let y = some_closure('d');
//     println!("x={:?}", x);
// }

// 如果我们想在包体外依然访问x，即x不是去所有权，怎么办？
// fn main() {
//     let mut x: String = String::from("abc");

//     // 在这里1.去掉move，包体内就对x进行了可变借用，而不是剥夺x的所有权。2.加一个作用域，目的是为了在作用域结束后让可变借用失效
//     {
//         let mut some_closure = |c: char| x.push(c);
//         some_closure('d');
//     }
//     println!("x={:?}", x);
// }

// 使用&符号借用(Borrowing)
// fn main() {
//     let x: Vec<i32> = vec![1i32, 2, 3];
//     let y = &x; //y借用x
//     println!("x={:?},y:{:?}", x, y);
// }

// fn main() {
//     let mut x: i32 = 100;
//     {
//         let y: &mut i32 = &mut x;
//         *y += 2;
//     }

//     println!("{}", x);
// }

// Borrowing也分为可变借用和不可变借用
// fn main() {
//     let x: Vec<i32> = vec![1i32, 2, 3, 4];

//     // 可以同时拥有多个不可变借用
//     let y = &x;
//     let z = &x;
//     let m = &x;

//     println!("{:?},{:?},{:?}", y, z, m);
// }
// 而同一时刻只能拥有一个可变借用(&mut T)，且被借用的变量本身必须有可变性：
// fn main() {
//     // 源变量x可变性
//     let mut x: Vec<i32> = vec![1i32, 2, 3];

//     // 只能有一个可变借用
//     let y = &mut x;

//     // let z = &mut x;//这里错误
//     y.push(100);

//     println!("{:?}", y);

//     // println!("{:?}", x);//这里错误，可变借用未释放，源变量不可访问
// }

fn main() {
    let mut x: Vec<i32> = vec![1i32, 2, 3];

    // 更新数组
    // push中对数组进行了可变借用，并在push函数退出时销毁这个借用
    x.push(10);

    {
        // 可变借用1
        let mut y = &mut x;
        y.push(100);

        // 可变借用2，注意：此处是对y的借用，不可再对x进行借用，因为y在此时依然存活
        let z = &mut y;
        z.push(1000);

        println!("{:?}", z);
    } //y和z在此时倍销毁，并释放借用

    // 此时访问x正常
    println!("{:?}", x);
}

// 借用不改变内存的所有者(Owner)，借用只是对源内存的临时引用。
// 在借用周期内，借用方可以读写这块内存，所有者被禁止读写内存；且所有者保证在有“借用”存在的情况下，不会释放或转移内存。
// 失去所有权的额变量不可以被借用(访问)
// 在租借期内，内存所有者保证不会释放、转移、可变租借这块内存，但如果是在非可变租借的情况下，所有者是允许继续非可变租借出去的。
// 借用周期满后，所有者收回读写权限
// 借用周期小于被借用者(所有者)的生命周期

// fn main() {
//     // <> 表示的是一个属于的关系,RefBoy这个结构体，不能比'a更长
//     struct RefBoy<'a> {
//         loc: &'a i32,
//     }
// }

// 结构体的引用字段必须要有显式的生命周期
// 一个被显式写出生命周期的结构体，与其自身的生命周期一定小于等于其显式写出的任意一个生命周期
// 生命周期是可以写多个的，用,分隔
// 生命周期与泛型都写在<>里，先生命周期后泛型，用,分隔

// #[derive(Copy, Clone)]
// struct A {
//     a: i32,
// }

// impl A {
//     pub fn show(&self) {
//         println!("{}", self.a);
//     }

//     pub fn add_two(&mut self) {
//         self.add_one();
//         self.add_one();
//         self.show();
//     }

//     pub fn add_one(&mut self) {
//         self.a += 1;
//     }
// }

// fn main() {
//     let mut ast = A { a: 12i32 };
//     ast.show();
//     ast.add_two();
// }

// enum SpecialPoint {
//     Point(i32, i32),
//     Special(String),
// }

// enum SpecialPoint {
//     Point { x: i32, y: i32 },
//     Special(String),
// }

// 枚举访问成员需要用到模式匹配
// enum SpecialPoint {
//     Point(i32, i32),
//     Special(String),
// }
// fn main() {
//     let sp = SpecialPoint::Point(0, 0);
//     match sp {
//         SpecialPoint::Point(x, y) => {
//             println!("I'am SpecialPoint(x={}, y={})", x, y);
//         }
//         SpecialPoint::Special(why) => {
//             println!("I'am Special because I am {}", why);
//         }
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// let point = Point{x:1,y:2};

// let Point{x:x,y:y} = point;

// let Point{x,y} = point;

fn use_str(s: &str) {
    println!("i am {}", s);
}

fn main() {
    let s = "Hello".to_string();
    use_str(&*s);
}

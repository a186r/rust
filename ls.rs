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

#[derive(Copy, Clone)]
struct A {
    a: i32,
}

impl A {
    pub fn show(&self) {
        println!("{}", self.a);
    }

    pub fn add_two(&mut self) {
        self.add_one();
        self.add_one();
        self.show();
    }

    pub fn add_one(&mut self) {
        self.a += 1;
    }
}

fn main() {
    let mut ast = A { a: 12i32 };
    ast.show();
    ast.add_two();
}

// 隐藏未使用代码警告的属性
#![allow(dead_code)]

enum Person {
    // unit-like(类单元结构体)
    Engineer,
    Scientist,
    // 元祖结构体
    Height(i32),
    Weight(i32),
    // 普通结构体
    Info { name: String, height: i32 },
}

// 此函数将一个Person作为参数，无返回值。
fn inspect(p: Person) {
    // enum的使用必须覆盖所有情形，所以使用match
    match p {
        Person::Engineer => println!("{} is enginner"),
        Person::Scientist => println!("is scientist"),
        Person::Height(i) => println!("has a height of {}.", i),
        Person::Weight(i) => println!("has a weight of {}", i),
        // 解构info
        Person::Info { name, height } => {
            println!("{} is {} tall", name, height);
        }
    }
}

fn main() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    // 'to_owned()'从一个字符串slice创建一个具有所有权的'String'
    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

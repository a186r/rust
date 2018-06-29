#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use Status::{Poor, Rich};

    // 自动的use Work内部的各个名称
    use Work::*;

    let status = Poor;

    let work = Civilian;

    match status {
        Rich => println!("rich rich rich"),
        Poor => println!("poor poor poor"),
    }

    match work {
        Civilian => println!("Civilian"),
        Soldier => println!("Soldier"),
    }
}

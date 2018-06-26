// // 这种结构体不能使用fmt::Display 或者fmt::Debug来进行打印
// struct Unprintable(i32);
// // derive会自动创建实现，借助fmt::Debug使得这个struct能够打印
// #[derive(Debug)]
// struct DebugPrintable(i32);
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("{:?} months is a year.", 12);
    println!(
        "{1:?}{0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    println!(
        "{:?} is first name and {0:?} is last name , {1:?} never used",
        "Hello", "World"
    );
}

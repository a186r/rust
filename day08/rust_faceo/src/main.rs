extern crate rust_faceo;
use rust_faceo::Draw;

#[derive(Debug)]
struct  SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// 在SelectBox上实现Draw trait
impl Draw for SelectBox{
    fn draw(&self) {

    }
}

use rust_faceo::{Screen,Button};

// 使用trait对象存储实现了相同trait的不同类型的值
fn main(){
    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no")
                ],
            }),

            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq,Debug)]
struct Shoe{
    size: u32,
    style: String,
}

// 使用filter方法和一个捕获shoe_size的闭包
// 获取一个鞋子vector的所有权和一个鞋子大小作为参数.返回一个只包含指定鞋子大小的vector
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

// The Gemini dollar is now global. @Bibox365 has announced plans to list the Gemini dollar! https://www.bibox.com/ 

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {size: 10,style: String::from("sneaker")},
        Shoe {size: 13,style: String::from("sandal")},
        Shoe {size: 10,style: String::from("boot")},
    ];

    let in_my_size = shoes_in_my_size(shoes,10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {size: 10,style: String::from("sneaker")},
            Shoe {size: 10,style: String::from("boot")},
        ] 
    );
}
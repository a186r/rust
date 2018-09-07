//字符串就是作为字节的集合外加一些方法实现的，String与其他集合也有不一样的地方，例如索引String是很复杂的，由于人和计算机理解String数据方式不同
// Rust核心语言中只有一种字符串类型，str，字符串slice，通常以借用的方式出现，&str
fn main() {
    // // 新建一个可变的空String
    // let mut s = String::new();

    // // 往里面装载数据
    // let data = "initial contents.";

    // let s = data.to_string();

    // let s = "i...c".to_string();

    // let s = String::new("ll.ss."); 

    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s = String::from("lo");
    // s.push("l");

    // let s1 = String::from("Hello,");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toc");

    // let s= format!("{}-{}-{}",s1,s2,s3);

    // println!("{}",s);

    // let len = String::from("Hola").len();

    // let hello = "Здравствуйте";

    // let s = &hello[0..2];

    // println!("{}",s);

    // 遍历字符串
    // for c in "नमस्ते".chars() {    
    // for c in "नमस्ते".bytes() {
    //     println!("{}",c);
    // }
}

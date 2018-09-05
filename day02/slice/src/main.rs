fn main() {

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // println!("xxx:{}",word);

// clear尝试获取一个可变引用，但是当拥有某个值的不可变引用时，就不能再获取它的可变引用了，所以这里Error
    // s.clear();

    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);

    println!("word:{}",word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    println!("word:{}",word);

    let word = first_word(my_string_literal);

    println!("word:{}",word);
}

// fn first_word(s: &String) -> usize{
//     let bytes = s.as_bytes();

//     for(i,&item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }

//     s.len()
// }

// 字符串slice的类型声明写作&str
// fn first_word(s: &String) -> &str{
fn first_word(s:&str) -> &str{

    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}
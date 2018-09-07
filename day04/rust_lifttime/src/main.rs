// fn main() {
//     // println!("Hello, world!");

//     // {
//     //     let r;
//     //     {
//     //         let x = 5;
//     //         r = &x;
//     //     }//x在这里就挂了

//     //     println!("r:{}",r);
//     // }

//     // let string1 = String::from("hello");
//     // let string2 = "world";

//     // let result = longest(string1.as_str(),string2);
//     // println!("The longest string is {}",result);

//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(),string2.as_str());

//         println!("The longest string is {}",result);
//     }
// }

// fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
//     if x.len() > y.len(){
//         x
//     } else {
//         y
//     }
// }

// fn longest<'a>(x: &'a str,y: &str) -> &'a str{
//     x
// }

// 存放引用的结构体
// struct  ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main(){
//     let novel = String::from("Call me Ishmael. Some years ago...");

//     let first_sentence = novel.split('.')
//         .next()
//         .expect("Could not find a '.'");
    
//     let i = ImportantExcerpt {part: first_sentence};
// }


// impl<'a> ImportantExcerpt<'a>{
//     fn level(&self) -> i32 {
//         3
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self,announcement: &str) -> &str {
//         println!("Attention please: {}",announcement);
//         self.part
//     }
// }


// let s: &'static str = "I have a static lifetime.";

use std::fmt::Display;

fn longest_with_an_announcement<'a,T>(x: &'a str,y:&'a str,ann: T) -> &'a str 
    where T: Display{
        println!("Announcement! {}",ann);
        if x.len() > y.len(){
            x
        } else {
            y
        }
    }
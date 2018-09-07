// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f{
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt"){
//                 Ok(fc) => fc,
//                 Err(e) => {
//                     panic!(
//                         "Tried to create file but there was a problem:{:?}",
//                         e
//                     )
//                 },
//             }
//         },

//         Err(error) => {
//             panic!(
//                 "There was a problem opening the file: {:?}",
//                 error
//             )
//         },
//     };
// }

// use std::fs::File;

// fn main(){
//     let f = File::open("hello.txt").unwrap();
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn main(){
//     read_username_from_file();
// }

// fn read_username_from_file() -> Result<String,io::Error>{
//     let f = File::open("hello.txt");

//     let mut f = match f{
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

// // 将文件中的内容读取到s中
//     match f.read_to_string(&mut s){
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn main(){

// }

// fn read_username_from_file() -> Result<String,io::Error>{
//     let mut f = File::open("Hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// 进一步缩短代码
// fn read_username_from_file() -> Result<String,io::Error>{
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     loop{
//         let guess: i32 = match guess.trim().parse(){
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         if guess < 1 || guess > 100{
//             println!(
//                 "The secret number will be between 1 and 100."
//             );
//             continue;
//         }
//         match guess.cmp(&secret_number){}

//     }
// }

pub struct Guess{
    value:u32,
}

impl Guess{
    pub fn new(value:u32) -> guess{
        if value < 1 || value > 100{
            panic!("Guess value must be between 1 and 100,got {}",value);
        }

        Guess {
            value
        }
    }

// getter
    pub fn value(&self) -> u32{
        self.value
    }
}
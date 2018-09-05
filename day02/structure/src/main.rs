// fn main() {

//     let mut user1 = User{
//         email:String::from("someone@gmail.com"),
//         username:String::from("someusername123"),
//         active:true,
//         sign_in_count:1,
//     };

//     let user2 = User{
//         email:String::from("anotheranother@gmail.com"),
//         username:String::from("anothersomeusername222");
//         ..user1
//     };

//     user1.email = String::from("anotheremail@gmail.com");

//     struct Color(i32,i32,i32);
//     struct Point(i32,i32,i32);
//     // black和origin值是不同的类型，因为他们是不同的元组结构体的实例。
//     let black = Color(0,0,0);
//     let origin = Point(0,0,0);
// }

// fn build_user(email:String,username:String) -> User{
//     User{
//         email:email,
//         username:username,
//         active:true,
//         sign_in_count:1,
//     }
// }

// fn build_user2(email:String,username:String) -> User{
//     User{
//         email,
//         username,
//         active:true,
//         sign_in_count:1,
//     }
// }

// 结构体数据的所有权
struct User{
    username:&str,
    email:&str,
    sign_in_count:u64,
    active:bool,
}

// 编译器会报错，需要生命周期标示符
fn main() {
    let user1 = User{
        email:"someone@gmail.com",
        username:"someusername",
        active:true,
        sign_in_count:1,
    };
}
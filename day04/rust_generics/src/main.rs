// fn main() {
//     let number_list = vec![1,2,3,4,5,6];

//     let mut largest = number_list[0];

//     for number in number_list{
//         if number > largest{
//             largest = number;
//         }
//     }

//     println!("The largest number is {}",largest);
// }

// fn largest(list: &[i32]) -> i32{
//     let mut largest = list[0];

//     for &number in list.iter(){
//         if number > largest{
//             largest = number;
//         }
//     }

//     largest
// }

// fn main(){
//     let number_list = vec![1,3,5,6,7,8,43,3];

//     let result = largest(&number_list);

//     println!("The Largest number is {}",result);

//     let number_list = vec![123,42145,654,765,785,32124,3];

//     let result = largest(&number_list);

//     println!("The Largest number is {}",result);

// }


// ----------------------------------------------------------泛型数据类型
// fn largest<T>(list: &[T]) -> T {}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter(){
//         if item > largest{
//             largest = item;
//         }
//     }

//     largest
// }

// fn main(){
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// struct Point<T,U>{
//     x:T,
//     y:U,
// }

// fn main(){
//     let both_integer = Point{x:2,y:3.4};
//     let both_float = Point{x:2,y:2};
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

struct Point<T,U>{
    x:T,
    y:U,
}

impl<T,U> Point<T,U> {
    fn mixup<V,W>(self,other: Point<V,W>) -> Point<T,W> {
        Point{
            x:self.x,
            y:other.y,
        }
    }
}

fn main(){
    let p1 = Point{x:2,y:2.2};
    let p2 = Point{x:"hel",y:'x'};

    let p3 = p1.mixup(p2);

    println!(
        "p3.x = {},p3.y = {}",
        p3.x,
        p3.y,
    );
}
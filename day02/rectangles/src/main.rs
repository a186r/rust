// 1.普通实现：函数area本来计算一个长方形面积，但是却有两个参数，这两个参数是关联的，但是这种实现方式却没有体现出这一点。
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixles.",
//         area(width1,height1)
//     );

// }

// fn main(){
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixles.",
//         width1 * height1
//     );
// }

// fn area(width: u32,height: u32) -> u32{
//     width * height
// }

// 2.使用元组重构，元组帮助我们增加了一些结构性，并且现在只需要传入一个参数，但是元组
// 并没有给出元素的名称，所以计算变得更费解了，因为不得不使用索引来获取元组的每一部分。
// fn main() {
//     let rect1 = (30,50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//     //直接将元组结构 
// fn area(dimensions:(u32,u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

// fn main() {
//     let rect1 = (30,50);

//     println!(
//         "The area of the rectangle is {} square pixles.",
//         area(rect1)
//     );
// }

// fn area(dimensions:(u32,u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

// 3.使用结构体重构，赋予更多意义
// 我们定义了一个结构体并称为Rectangle。在{}中定义了width和height，都是u32类型
// area函数访问Rectangle的width和height字段，area的签名现在明确的表明了我们的
// 意图：通过width和height字段计算出一个Rectangle的面积。结构体胜在更清晰明了。
// struct Rectangle{
//     width:u32,
//     height:u32,
// }

// fn main(){
    
//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

// }

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// struct Rectangle{
//     width:u32,
//     height:u32,
// }

// fn main(){

//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// 4.通过派生trait增加实用功能
// 这里使用derive注解来派生Debug trait，并使用调试格式打印Rectangle实例
// #[derive(Debug)]
// struct Rectangle{
//     width: u32,
//     height:u32,
// }
// // 在Rectangle结构体上定义area方法
// impl Rectangle {
//     // add code here
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
// }

// fn main() {
    
//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     // println!("rect1 is {:#?}",rect1);
//     println!(
//         "The area of rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32,
// }

// impl Rectangle{
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
// }

// fn main(){
//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     println!("rect1 is {:#?}",rect1);

//     println!(
//         "The area of rectangle is {} square pixels.",
//         rect1.area()
//     )
// }

// 5.增加更多实用功能
// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32,
// }

// // 在Rectangle结构体上定义area方法和can_hold方法
// impl Rectangle {
//     // add code here
//     fn area(&self) -> u32{
//         self.width * self.height
//     }

//     fn can_hold(&self,other: &Rectangle) -> bool{
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
    
//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     let rect2 = Rectangle{
//         width:10,
//         height:20
//     };

//     let rect3 = Rectangle{
//         width:40,
//         height:90
//     };

//     println!(
//         "area is {}",
//         rect1.area()
//     );

//     println!(
//         "Can rect1 hold rect2? {}",
//         rect1.can_hold(&rect2)
//     );

//     println!(
//         "Can rect1 hold rect3? {}",
//         rect1.can_hold(&rect3)
//     );
// }

// impl块的另一个有用的功能是：允许在impl块中定义不以self作为参数的函数，这杯称为关联函数
// 因为它们与结构体相关联，即便如此它们仍然是函数而不是方法，因为它们并不作用于一个结构体的实例。
// 我们使用过的String::from就是一个关联函数。

// 正方形
// imple Rectangle{
//     fn square(size:u32) -> Rectangle{
//         Rectangle{
//             width:size,
//             height:size
//         }
//     }
// }

// 可以使用多个impl块，虽然没有理由将这些方法分散在多个impl块中，但是这是有效的语法
// impl Rectangle{
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
// }

// impl Rectangle{
//     fn area(&self,other: &Rectangle) -> bool{
//         self.width < other.width && self.height < other.height
//     }
// }

// 结构体并不是创建自定义类型的唯一方法；让我们转向Rust的枚举功能并为自己的工具箱再添一个工具！mmp哦，又要添一个工具，辣鸡哦，我回家再看吧，这都要下班了，还工具工具工具呢，滚了哦